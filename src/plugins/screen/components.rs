use bevy::prelude::*;

/// Represents the MCDU's left title
#[derive(Component)]
pub struct LeftTitle;

/// Represents a single cell of text rendered on the MCDU's main content section
#[derive(Component)]
pub struct MainContentCell {
    pub row_index: usize,
    pub col_index: usize,
    pub is_label: bool,
}

impl MainContentCell {
    pub fn new(row_index: usize, col_index: usize, is_label: bool) -> Self {
        MainContentCell {
            row_index,
            col_index,
            is_label,
        }
    }
}

/// Represents the MCDU's scratchpad
#[derive(Component)]
pub struct Scratchpad;
