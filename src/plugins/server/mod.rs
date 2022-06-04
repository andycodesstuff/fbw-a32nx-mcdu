pub mod systems;

use crate::{plugins::server::systems::setup, utils::graph::Graph};
use bevy::prelude::*;
use serde::Deserialize;

/// Represents an update that has to be drawn on the MCDU screen
#[derive(Debug)]
pub struct ScreenUpdate {
    pub lines: Vec<Vec<ParsedText>>,
    pub scratchpad: ParsedText,
    pub title: ParsedText,
    pub title_left: ParsedText,
    pub arrows: Vec<bool>,
}

/// A parsed text is a tree-like representation (similar to the DOM) that describes how text
/// should be segmented in sections, each with their formatting and content
type ParsedText = Graph<String, TextVertex, bool>;

#[derive(Debug, Hash)]
pub struct TextVertex {
    pub formatter: TextFormatter,
    pub value: Option<String>,
}

impl Default for TextVertex {
    fn default() -> Self {
        TextVertex {
            formatter: TextFormatter::End,
            value: None,
        }
    }
}

/// Represents the various text formatters that can be used on the MCDU screen
#[derive(Debug, Hash, PartialEq)]
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

/// Represents the event associated with a screen update request
pub struct ScreenUpdateEvent(pub ScreenUpdate);

/// Represents the message sent to the server when a screen update is requested by the client
#[derive(Debug, Deserialize)]
struct ScreenUpdateMessage {
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
    arrows: Vec<bool>,
}

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScreenUpdateEvent>()
            .add_startup_system(setup);
    }
}