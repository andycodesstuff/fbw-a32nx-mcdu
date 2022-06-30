use super::{ScreenState, TextFormatter, TextVertex};
use crate::{
    plugins::server::{ScreenUpdate, ScreenUpdateEvent, ScreenUpdateMessage},
    utils::graph::Graph,
};
use bevy::prelude::*;
use regex::Regex;
use std::fs;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

/// Set-ups the WebSocket server to accept connections
pub fn setup(mut events: EventWriter<ScreenUpdateEvent>) {
    events.send(ScreenUpdateEvent(load_test_message()));
}

fn load_test_message() -> ScreenUpdate {
    let path = "test_message.json";
    let mut json_msg = fs::read_to_string(path).unwrap();

    // Replace unrenderable unicode character used as whitespace with a simple space
    let nbsp_regex = Regex::new(r"\u00A0").unwrap();
    json_msg = nbsp_regex.replace_all(&json_msg, " ").to_string();

    let raw_screen_update = parse_json_msg(&json_msg).unwrap();
    ScreenUpdate {
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
                    .collect::<Vec<Graph<String, TextVertex, bool>>>();
                line.swap(1, 2);

                line
            })
            .collect(),
        scratchpad: parse_raw_text(raw_screen_update.scratchpad),
        title: parse_raw_text(raw_screen_update.title),
        title_left: parse_raw_text(raw_screen_update.title_left),
        arrows: raw_screen_update.arrows,
    }
}

/// Parses the message in JSON format sent to the server
fn parse_json_msg(json: &str) -> Option<ScreenState> {
    match serde_json::from_str::<ScreenUpdateMessage>(json) {
        Ok(msg) => Some(msg.left),
        _ => None,
    }
}

/// Parses the formatter tags used by the FlyByWire's A32NX mod and returns a tree-like
/// representation of how text should be segmented in sections, each with their formatting and
/// content
fn parse_raw_text(raw_text: String) -> Graph<String, TextVertex, bool> {
    let mut current_text: String = raw_text;
    let mut current_parent: String = "root".to_string();
    let mut position = 0;

    // Create a new graph with a single node to use as root
    let mut graph: Graph<String, TextVertex, bool> = Graph::new();
    graph.push_vertex(current_parent.clone(), TextVertex::default());

    while current_text.graphemes(true).count() > 0 {
        // Look for the beginning of a formatter tag
        let formatter_re = Regex::new(r"^(\{(?P<formatter>[a-zA-Z]+)\}(?P<rest>.*))").unwrap();
        match formatter_re.captures(current_text.as_str()) {
            Some(captures) => {
                // Split the formatter from the rest of the string
                let formatter = TextFormatter::from_str(&captures["formatter"]);
                let rest = &captures["rest"];

                // Update current references and skip to the next iteration if end tag is found
                if formatter == TextFormatter::End {
                    current_text = rest.to_string();
                    current_parent = graph.get_parent(current_parent.clone()).unwrap();
                    continue;
                }

                // Create a new vertex
                let vertex_id = Uuid::new_v4().to_string();
                let mut vertex = TextVertex {
                    formatters: vec![formatter],
                    value: None,
                    ..default()
                };

                // Inherit formatters from parent node
                if let Some(parent) = graph.get_vertex(current_parent.clone()) {
                    for formatter in parent.formatters.iter() {
                        vertex.formatters.push(formatter.clone());
                    }
                }

                // Register the vertex and its edges
                graph.push_vertex(vertex_id.clone(), vertex);
                graph.push_edge(current_parent.clone(), vertex_id.clone(), false);
                graph.push_edge(vertex_id.clone(), current_parent.clone(), true);

                current_text = rest.to_string();
                current_parent = vertex_id;
            }
            None => {
                // Extract the text to associate to the vertex that is currently being parsed
                let value: String = current_text
                    .graphemes(true)
                    .take_while(|c| *c != "{")
                    .collect();
                let value_len: usize = value.graphemes(true).map(|c| c.bytes().count()).sum();

                // Update the vertex
                let mut vertex = graph.get_vertex_mut(current_parent.clone()).unwrap();
                vertex.value = Some(value);
                vertex.position = position;
                position += 1;

                // Continue parsing after the text that has just been extracted
                let section = &current_text[(if value_len > 0 { value_len } else { 0 })..];
                current_text = section.to_string();
            }
        }
    }

    graph
}
