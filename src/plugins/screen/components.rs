use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Describes the state of the MCDU screen
///
/// The UI of the screen is divided into a grid composed of rows (Line components) and 3
/// columns: left, center and right (represented using the Cell component)
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
    pub is_label: bool,
}

impl Default for Line {
    fn default() -> Self {
        Line { is_label: false }
    }
}

/// Describes the state of a cell of information on the MCDU screen
#[derive(Component, Inspectable)]
pub struct Cell {
    pub row_index: usize,
    pub col_index: usize,
    pub text: Text,
    pub is_label: bool,
}

impl Cell {
    pub fn new(row_index: usize, col_index: usize, is_label: bool) -> Self {
        Cell {
            row_index,
            col_index,
            text: match col_index {
                0 => Text {
                    sections: vec![],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Left,
                    },
                },
                1 => Text {
                    sections: vec![],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                },
                2 => Text {
                    sections: vec![],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Right,
                    },
                },
                _ => Text::default(),
            },
            is_label,
        }
    }
}
