use crate::plugins::server::{ScreenUpdate, ScreenUpdateEvent, ScreenUpdateMessage};
use bevy::prelude::*;
use std::fs;

/// Set-ups the WebSocket server to accept connections
pub fn setup(mut events: EventWriter<ScreenUpdateEvent>) {
    events.send(ScreenUpdateEvent(load_test_message()));
}

fn load_test_message() -> ScreenUpdate {
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
