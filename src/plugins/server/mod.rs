pub mod systems;

use crate::plugins::server::systems::{events_relay, setup};
use bevy::prelude::*;
use crossbeam_channel::Receiver;
use serde::Deserialize;

/// Represents an update that has to be drawn on the MCDU screen
#[derive(Debug)]
pub struct ScreenUpdate {
    pub lines: Vec<Vec<ParsedText>>,
    pub scratchpad: ParsedText,
    pub title: ParsedText,
    pub title_left: ParsedText,
    pub page: ParsedText,
    pub arrows: Vec<bool>,
}

/// Describes how text should be segmented into sections, each with their owm formatting and
/// content
pub type ParsedText = Vec<TextSegment>;

#[derive(Debug)]
pub struct TextSegment {
    pub formatters: Vec<TextFormatter>,
    pub value: String,
}

/// Represents the various text formatters that can be used on the MCDU screen
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum TextFormatter {
    AlignLeft,
    AlignRight,
    ColorAmber,
    ColorCyan,
    ColorGreen,
    ColorInop,
    ColorMagenta,
    ColorRed,
    ColorWhite,
    ColorYellow,
    End,
    FontBig,
    FontSmall,
    Space,
}

impl TextFormatter {
    pub fn from_str(str: &str) -> Self {
        match str {
            "left" => TextFormatter::AlignLeft,
            "right" => TextFormatter::AlignRight,
            "amber" => TextFormatter::ColorAmber,
            "cyan" => TextFormatter::ColorCyan,
            "green" => TextFormatter::ColorGreen,
            "inop" => TextFormatter::ColorInop,
            "magenta" => TextFormatter::ColorMagenta,
            "red" => TextFormatter::ColorRed,
            "white" => TextFormatter::ColorWhite,
            "yellow" => TextFormatter::ColorYellow,
            "big" => TextFormatter::FontBig,
            "small" => TextFormatter::FontSmall,
            "sp" => TextFormatter::Space,
            _ => TextFormatter::End,
        }
    }
}

#[derive(Deref)]
pub struct ScreenUpdateReceiver(Receiver<ScreenUpdate>);
/// Represents the event associated with a screen update request
pub struct ScreenUpdateEvent(pub ScreenUpdate);

/// Represents the message sent to the server when a screen update is requested by the client
#[derive(Debug, Deserialize)]
struct ScreenUpdateMessage {
    #[allow(dead_code)]
    right: ScreenState,
    left: ScreenState,
}
#[derive(Debug, Deserialize)]
struct ScreenState {
    lines: Vec<Vec<String>>,
    scratchpad: String,
    title: String,
    #[serde(alias = "titleLeft")]
    title_left: String,
    page: String,
    arrows: Vec<bool>,
}

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScreenUpdateEvent>()
            .add_startup_system(setup)
            .add_system(events_relay);
    }
}
