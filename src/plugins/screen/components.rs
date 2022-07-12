use bevy::prelude::*;

/// Represents a row of information on the MCDU's screen. Contains the index of the current row
#[derive(Component)]
pub struct Row {
    pub row_index: usize,
    pub is_label: bool,
}

impl Row {
    pub fn new(row_index: usize, is_label: bool) -> Self {
        Self {
            row_index,
            is_label,
        }
    }
}

/// A specialisation of the row component, represents the header section of the MCDU's screen
/// where the page title and page indicator (if available) are shown
#[derive(Component)]
pub struct RowHeader;

/// A specialisation of the row component, represents the main content displayed on the MCDU's
/// screen
#[derive(Component)]
pub struct RowContent;

/// A specialisation of the row component, represents the bottom section of the MCDU's screen
/// where the scratchpad and the vertical scroll indicator (if available) are shown
#[derive(Component)]
pub struct RowFooter;
