use crate::{
    plugins::{
        screen::{components::Cell, components::Screen},
        server::{ParsedText, ScreenUpdateEvent, TextFormatter, TextSegment},
    },
    SCREEN_PADDING, SCREEN_ROWS,
};
use bevy::prelude::*;
use rand::Rng;

use super::components::Scratchpad;

/// Set-ups the UI hierarchy of the MCDU and the elements that will be populated with data
pub fn setup(mut commands: Commands) {
    let row_height = 100.0 / (SCREEN_ROWS as f32);
    let mut rng = rand::thread_rng();

    // Root container
    let screen = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::ColumnReverse,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: UiColor(Color::rgb_u8(0x0d, 0x14, 0x23)),
            ..default()
        })
        .insert(Screen::default())
        .id();

    // Screen rows
    for row_index in 0..SCREEN_ROWS {
        let is_label = row_index % 2 != 0;

        // Screen columns
        for col_index in 0..3 {
            commands
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        justify_content: match col_index {
                            0 => JustifyContent::FlexStart,
                            1 => JustifyContent::Center,
                            2 => JustifyContent::FlexEnd,
                            _ => JustifyContent::FlexStart,
                        },
                        position: Rect {
                            left: Val::Px(0.0),
                            right: Val::Px(0.0),
                            top: Val::Percent(row_height * (row_index as f32)),
                            bottom: Val::Auto,
                        },
                        padding: Rect {
                            left: Val::Px(SCREEN_PADDING),
                            right: Val::Px(SCREEN_PADDING),
                            top: Val::Undefined,
                            bottom: Val::Undefined,
                        },
                        size: Size::new(Val::Percent(100.0), Val::Percent(row_height)),
                        ..default()
                    },
                    color: UiColor(Color::rgba(
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                        0.0,
                    )),
                    ..default()
                })
                .with_children(|parent| {
                    let mut text_cell = parent.spawn_bundle(TextBundle {
                        style: Style {
                            // I have no idea why specifying an aspect ratio aligns the
                            // content properly but yeah, here it is
                            aspect_ratio: Some(1.0),
                            ..default()
                        },
                        ..default()
                    });

                    if row_index == 0 {
                        // Title row
                    } else if row_index == SCREEN_ROWS - 1 {
                        // Scratchpad row
                        match col_index {
                            #[rustfmt::skip]
                            0 => { text_cell.insert(Scratchpad); }
                            _ => {}
                        };
                    } else {
                        // Content rows
                        text_cell.insert(Cell::new(row_index - 1, col_index, is_label));
                    }
                })
                .insert(Parent(screen));
        }
    }
}

/// Updates the UI of the MCDU screen with the data coming from the simulator
pub fn update_screen(
    mut cells_q: Query<(&Cell, &mut Text), With<Cell>>,
    mut scratchpad_q: Query<&mut Text, (Without<Cell>, With<Scratchpad>)>,
    mut events: EventReader<ScreenUpdateEvent>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    for screen_update_event in events.iter() {
        let screen_update = &screen_update_event.0;
        let window = windows.get_primary().unwrap();
        let window_height = window.height();

        // Compute the base font size for any text
        let font_size = (window_height - SCREEN_PADDING * 3.5) / (SCREEN_ROWS as f32);

        // Update the screen's content
        for (cell, mut text) in cells_q.iter_mut() {
            let parsed_text = &screen_update.lines[cell.row_index][cell.col_index];
            text.sections = build_text_sections(parsed_text, cell.is_label, &asset_server);
            text.sections
                .iter_mut()
                .for_each(|section| section.style.font_size = font_size);
        }

        // Update the scratchpad
        let mut scratchpad_text = scratchpad_q.get_single_mut().unwrap();
        scratchpad_text.sections =
            build_text_sections(&screen_update.scratchpad, false, &asset_server);
        scratchpad_text
            .sections
            .iter_mut()
            .for_each(|section| section.style.font_size = font_size);
    }
}

/// Builds the text sections of a Text component given how text should be segmented and styled
fn build_text_sections(
    parsed_text: &ParsedText,
    is_label: bool,
    asset_server: &Res<AssetServer>,
) -> Vec<TextSection> {
    let mut text_sections: Vec<TextSection> = Vec::new();

    for TextSegment { formatters, value } in parsed_text {
        let mut font_name = if is_label {
            "HoneywellMCDUSmall.ttf"
        } else {
            "HoneywellMCDU.ttf"
        };
        let mut color = Color::rgb_u8(0xff, 0xff, 0xff);

        for formatter in formatters {
            // TODO: Handle AlignLeft, AlignRight

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
        }

        text_sections.push(TextSection {
            value: value.clone(),
            style: TextStyle {
                font: asset_server.load(font_name),
                font_size: 0.0,
                color,
            },
        });
    }

    text_sections
}
