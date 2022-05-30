use crate::plugins::screen::SCREEN_COLUMNS;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Describes the state of the MCDU screen
#[derive(Component, Inspectable)]
pub struct Screen {
    arrows: [bool; 4],
    page: String,
    scratchpad: String,
    title: String,
    title_left: String,
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            arrows: [false, false, false, false],
            page: "".to_string(),
            scratchpad: "".to_string(),
            title: "".to_string(),
            title_left: "".to_string(),
        }
    }
}

/// Describes the state of a row of information in the MCDU screen
#[derive(Component, Inspectable)]
pub struct Line {
    pub columns: [String; SCREEN_COLUMNS],
    pub is_label: bool,
}

impl Default for Line {
    fn default() -> Self {
        Line {
            columns: ["".to_string(), "".to_string(), "".to_string()],
            is_label: true,
        }
    }
}
