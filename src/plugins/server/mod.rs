use serde::Deserialize;
use std::fs;

/// Represents an update that has to be drawn on the MCDU screen
#[derive(Debug, Deserialize)]
pub struct ScreenUpdate {
    pub lines: Vec<Vec<String>>,
    pub scratchpad: String,
    pub title: String,
    #[serde(alias = "titleLeft")]
    pub title_left: String,
    pub arrows: Vec<bool>,
}

/// Represents the message sent to the server when a screen update is requested by the client
#[derive(Debug, Deserialize)]
struct ScreenUpdateMessage {
    right: ScreenUpdate,
    left: ScreenUpdate,
}

pub fn load_test_message() -> ScreenUpdate {
    let path = "test_message.json";
    let json_msg = fs::read_to_string(path).unwrap();

    parse_json_msg(&json_msg).unwrap()
}

/// Parses the message in JSON format sent to the server
fn parse_json_msg(json: &str) -> Option<ScreenUpdate> {
    match serde_json::from_str::<ScreenUpdateMessage>(json) {
        Ok(msg) => Some(msg.left),
        _ => None,
    }
}
