use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Describes the state of the MCDU screen
///
/// The UI of the screen is divided into a grid composed of rows (Line components) and 3
/// columns: left, right and center (represented using the Cell component. Note that the order
/// in which they were described is relevant as column index 0 will be the left-most cell, 1
/// would be the right-most cell and 2 would be the center cell. This behaviour is inherited
/// from the FBW A32NX mod and how it formats data sent to the MCDU server)
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

/// Describes the state of a cell of information on the MCDU screen
#[derive(Component)]
pub struct Cell {
    pub row_index: usize,
    pub col_index: usize,
    pub is_label: bool,
}

impl Cell {
    pub fn new(row_index: usize, col_index: usize, is_label: bool) -> Self {
        Cell {
            row_index,
            col_index,
            is_label,
        }
    }
}

/// Describes the state of the MCDU's left title
#[derive(Component)]
pub struct LeftTitle;

/// Describes the state of the MCDU's scratchpad
#[derive(Component)]
pub struct Scratchpad;
