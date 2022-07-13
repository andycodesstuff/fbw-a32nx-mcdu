use crate::{
    plugins::server::{ParsedText, TextFormatter, TextSegment},
    SCREEN_COLS, SCREEN_ROWS,
};
use bevy::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

const FONT_ASPECT_RATIO: f32 = 1.3850;
const FONT_SIZE_PERCENT: f32 = 0.90;

/// Computes the font size given the window where text will be displayed
pub(super) fn compute_font_size(window: &Window) -> f32 {
    let window_height = window.height();
    let font_size = window_height / (SCREEN_ROWS as f32) * FONT_SIZE_PERCENT;

    font_size
}

/// Computes the horizontal whitespace between the end of a grapheme and the start of the next
/// one
pub(super) fn compute_font_whitespace(font_size: f32) -> f32 {
    font_size - (font_size / FONT_ASPECT_RATIO)
}

/// Computes the width of a single row based on the font size and the n. of columns we have to
/// display
pub(super) fn compute_row_width(font_size: f32) -> f32 {
    (font_size / FONT_ASPECT_RATIO) * (SCREEN_COLS as f32)
}

#[derive(Clone, Copy)]
pub(super) enum TextAlign {
    Left,
    Center,
    Right,
}

/// Computes all the TextBundles that make up the given parsed text
pub(super) fn compute_text_bundles(
    parsed_text: &ParsedText,
    default_alignment: TextAlign,
    is_label_row: bool,
    asset_server: &AssetServer,
    window: &Window,
) -> Vec<TextBundle> {
    let mut text_bundles: Vec<TextBundle> = Vec::new();
    let mut left_text_sections: Vec<TextSection> = Vec::new();
    let mut center_text_sections: Vec<TextSection> = Vec::new();
    let mut right_text_sections: Vec<TextSection> = Vec::new();

    let font_size = compute_font_size(window);
    let font_whitespace = compute_font_whitespace(font_size);
    let row_width = compute_row_width(font_size);

    for TextSegment { formatters, value } in parsed_text {
        let mut font_name = if is_label_row {
            "HoneywellMCDUSmall.ttf"
        } else {
            "HoneywellMCDU.ttf"
        };
        let mut color = Color::rgb_u8(0xff, 0xff, 0xff);
        let mut align = default_alignment;

        for formatter in formatters {
            // Extract which font to use
            font_name = match formatter {
                TextFormatter::FontBig => "HoneywellMCDU.ttf",
                TextFormatter::FontSmall => "HoneywellMCDUSmall.ttf",
                _ => font_name,
            };

            // Extract which color to use
            color = match formatter {
                TextFormatter::ColorAmber => Color::rgb_u8(0xff, 0x9a, 0x00),
                TextFormatter::ColorCyan => Color::rgb_u8(0x00, 0xff, 0xff),
                TextFormatter::ColorGreen => Color::rgb_u8(0x00, 0xff, 0x00),
                TextFormatter::ColorInop => Color::rgb_u8(0x66, 0x66, 0x66),
                TextFormatter::ColorMagenta => Color::rgb_u8(0xff, 0x94, 0xff),
                TextFormatter::ColorRed => Color::rgb_u8(0xff, 0x00, 0x00),
                TextFormatter::ColorWhite => Color::rgb_u8(0xff, 0xff, 0xff),
                TextFormatter::ColorYellow => Color::rgb_u8(0xff, 0xff, 0x00),
                _ => color,
            };

            // Extract which alignment to use
            match formatter {
                #[rustfmt::skip]
                TextFormatter::AlignLeft => { align = TextAlign::Left; }
                #[rustfmt::skip]
                TextFormatter::AlignRight => { align = TextAlign::Right; }
                _ => {}
            };
        }

        // Group together text sections based on their alignment
        let text_section = TextSection {
            value: value.clone(),
            style: TextStyle {
                font: asset_server.load(font_name),
                font_size,
                color,
            },
        };

        match align {
            TextAlign::Left => left_text_sections.push(text_section),
            TextAlign::Center => center_text_sections.push(text_section),
            TextAlign::Right => right_text_sections.push(text_section),
        };
    }

    let create_text_bundle = |align, sections: Vec<TextSection>| -> TextBundle {
        let left_pos = match align {
            TextAlign::Left => Val::Px(font_whitespace),
            TextAlign::Center => {
                // Compute the horizontal size (in pixels) of the text to be rendered
                let text_length: usize = sections
                    .iter()
                    .map(|s| s.value.graphemes(true).count())
                    .sum();
                let text_size = (text_length as f32) * (font_size / FONT_ASPECT_RATIO);

                Val::Px(font_whitespace + (row_width / 2.0) - (text_size / 2.0))
            }
            _ => Val::Undefined,
        };
        let right_pos = match align {
            TextAlign::Right => Val::Px(0.0),
            _ => Val::Undefined,
        };

        TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: left_pos,
                    right: right_pos,
                    top: Val::Undefined,
                    bottom: Val::Undefined,
                },
                ..default()
            },
            text: Text {
                sections,
                ..default()
            },
            ..default()
        }
    };

    if !left_text_sections.is_empty() {
        text_bundles.push(create_text_bundle(TextAlign::Left, left_text_sections));
    }
    if !center_text_sections.is_empty() {
        text_bundles.push(create_text_bundle(TextAlign::Center, center_text_sections));
    }
    if !right_text_sections.is_empty() {
        text_bundles.push(create_text_bundle(TextAlign::Right, right_text_sections));
    }

    text_bundles
}
