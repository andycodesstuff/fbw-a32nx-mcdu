use super::{ParsedText, ScreenState, ScreenUpdateReceiver, TextFormatter, TextSegment};
use crate::plugins::server::{ScreenUpdate, ScreenUpdateEvent, ScreenUpdateMessage};
use bevy::prelude::*;
use crossbeam_channel::{unbounded, Sender};
use futures_util::{future, StreamExt, TryStreamExt};
use regex::Regex;
use std::{collections::VecDeque, fs};
use tokio::{
    net::{TcpListener, TcpStream},
    runtime::Builder,
};
use tokio_tungstenite::tungstenite::Message;
use unicode_segmentation::UnicodeSegmentation;

const FORMATTERS: &str =
    r"\{(?P<formatter>left|right|amber|cyan|green|inop|magenta|red|white|yellow|big|small|end)\}";
const SPACE_FORMATTER: &str = r"\{sp\}";
pub const WS_SERVER_ADDR: &str = "127.0.0.1:8380";

/// Set-ups the WebSocket server to accept connections
pub fn setup(mut commands: Commands) {
    let (tx, rx) = unbounded::<ScreenUpdate>();

    // Start the WebSocket server on a different thread
    std::thread::spawn(move || {
        Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap()
            .block_on(ws_server_runtime(tx));
    });

    commands.insert_resource(ScreenUpdateReceiver(rx));
}

/// Relays events generated by the WebSocket server to the bevy thread
pub fn events_relay(
    receiver: ResMut<ScreenUpdateReceiver>,
    mut events: EventWriter<ScreenUpdateEvent>,
) {
    for mcdu_event in receiver.try_iter() {
        events.send(ScreenUpdateEvent(mcdu_event));
    }
}

/// Set-ups the WebSocket server used to communicate with the MCDU
async fn ws_server_runtime(tx: Sender<ScreenUpdate>) {
    // Create the TCP listener and event loop that will accept connections
    let listener = TcpListener::bind(WS_SERVER_ADDR)
        .await
        .expect("Failed to bind");
    info!("Listening on {}", WS_SERVER_ADDR);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, tx.clone()));
    }
}

/// Accepts a new WebSocket connection and handles the client/server communication
async fn handle_connection(stream: TcpStream, tx: Sender<ScreenUpdate>) {
    // Accept a new WebSocket connection
    let remote_addr = stream.peer_addr().unwrap();
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Failed to handshake");
    info!("New WebSocket connection from {}", remote_addr);

    let (_write, read) = ws_stream.split();

    // Handle incoming messages
    read.try_for_each(|ws_message| {
        if let Message::Text(msg) = ws_message {
            // Extract the command and (optional) data from the message sent by the MCDU
            let mut sections = msg.splitn(2, ":").collect::<VecDeque<&str>>();
            let (_command, data) = (sections.pop_front(), sections.pop_back());
            info!("MCDU message: {:?}", _command);

            // Handle commands
            if let Some(command) = _command {
                match command {
                    "update" => handle_update_command(tx.clone(), data),
                    _ => {}
                };
            }
        }

        future::ready(Ok(()))
    })
    .await
    .expect("Failed to handle WebSocket message");
}

/// Handles the "update" command sent by the MCDU
fn handle_update_command(tx: Sender<ScreenUpdate>, data: Option<&str>) {
    if data.is_none() {
        warn!("Missing update data");
        return;
    }

    // Replace unrenderable unicode character used as whitespace with a simple space
    let nbsp_regex = Regex::new(r"\u00A0").unwrap();
    let json_msg = nbsp_regex.replace_all(data.unwrap(), " ").to_string();

    // Construct and send the screen update event
    let raw_screen_update = parse_json_msg(&json_msg).unwrap();
    let screen_update = ScreenUpdate {
        lines: raw_screen_update
            .lines
            .iter()
            .map(|line| {
                // Parse the line and swap the right and center column (FlyByWire's A32NX mod
                // uses the following layout [left, right, center] to represent a line whereas
                // in this project I prefer to use [left, center, right])
                let mut line = line
                    .iter()
                    .map(|section| parse_raw_text(section.clone()))
                    .collect::<Vec<ParsedText>>();
                line.swap(1, 2);

                line
            })
            .collect(),
        scratchpad: parse_raw_text(raw_screen_update.scratchpad),
        title: parse_raw_text(raw_screen_update.title),
        title_left: parse_raw_text(raw_screen_update.title_left),
        arrows: raw_screen_update.arrows,
    };

    tx.send(screen_update).unwrap();
}

/// Parses the message in JSON format sent to the server
fn parse_json_msg(json: &str) -> Option<ScreenState> {
    match serde_json::from_str::<ScreenUpdateMessage>(json) {
        Ok(msg) => Some(msg.left),
        _ => None,
    }
}

/// Parses the formatter tags used by the FlyByWire's A32NX mod
fn parse_raw_text(raw_text: String) -> ParsedText {
    let formatter_begin_re = Regex::new(format!("^({FORMATTERS}(?P<rest>.*))").as_str()).unwrap();
    let formatter_end_re = Regex::new(format!("(?P<rest>.*?){FORMATTERS}").as_str()).unwrap();
    let space_formatter_re = Regex::new(SPACE_FORMATTER).unwrap();

    let mut formatters_stack: Vec<TextFormatter> = Vec::new();
    let mut result: ParsedText = Vec::new();
    let mut current_text: String = raw_text;

    // Escape all {sp} self-closing tags with a whitespace
    current_text = space_formatter_re
        .replace_all(current_text.as_str(), " ")
        .to_string();

    while current_text.graphemes(true).count() > 0 {
        match formatter_begin_re.captures(current_text.as_str()) {
            Some(captures) => {
                // Split the formatter from the rest of the string
                let formatter = TextFormatter::from_str(&captures["formatter"]);
                let rest = &captures["rest"];

                // Push or pop the stack based on the formatter found
                if formatter == TextFormatter::End {
                    formatters_stack.pop();
                } else {
                    formatters_stack.push(formatter);
                }

                // Process the rest of the text
                current_text = rest.to_string();
            }
            None => {
                let mut value: String = String::from("");
                let mut value_len: usize = 0;

                // Extract the content of the text segment
                if let Some(captures) = formatter_end_re.captures(current_text.as_str()) {
                    value = captures["rest"].to_string();
                    value_len = value.graphemes(true).map(|c| c.bytes().count()).sum();
                }

                // Save the text segment
                result.push(TextSegment {
                    formatters: formatters_stack.clone(),
                    value: value,
                });

                // Process the rest of the text
                let next_text = &current_text[(if value_len > 0 { value_len } else { 0 })..];
                current_text = next_text.to_string();
            }
        }
    }

    result
}

#[allow(dead_code)]
fn load_test_message(tx: Sender<ScreenUpdate>) {
    let path = "test_message.json";
    let json_msg = fs::read_to_string(path).unwrap();

    handle_update_command(tx, Some(&json_msg));
}
