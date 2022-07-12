use crate::SCREEN_ROWS;
use bevy::prelude::*;

const FONT_SIZE_PERCENT: f32 = 0.90;

/// Computes the font size given the window where text will be displayed
pub(super) fn compute_font_size(window: &Window) -> f32 {
    let window_height = window.height();
    let font_size = window_height / (SCREEN_ROWS as f32) * FONT_SIZE_PERCENT;

    font_size
}
