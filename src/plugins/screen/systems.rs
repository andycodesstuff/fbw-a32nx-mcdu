use crate::{
    plugins::{
        screen::{
            components::Cell, components::Line, components::Screen, SCREEN_COLUMNS, SCREEN_LINES,
            SCREEN_PADDING,
        },
        server::{ScreenUpdateEvent, TextFormatter, TextVertex},
    },
    utils::graph::Graph,
    DEFAULT_HEIGHT,
};
use bevy::prelude::*;

/// Set-ups the UI hierarchy of the MCDU and the elements that will be populated with data
pub fn setup(mut commands: Commands) {
    // Root container
    let screen = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                padding: Rect::all(Val::Percent(SCREEN_PADDING)),
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .insert(Screen::default())
        .id();

    // Screen lines
    for row_index in 0..SCREEN_LINES {
        let is_label = row_index % 2 == 0;

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    align_items: if is_label {
                        AlignItems::FlexStart
                    } else {
                        AlignItems::Center
                    },
                    justify_content: JustifyContent::SpaceBetween,
                    flex_grow: 1.0,
                    ..default()
                },
                color: UiColor(Color::BLACK),
                ..default()
            })
            .with_children(|line| {
                // Line columns
                for col_index in 0..SCREEN_COLUMNS {
                    line.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![],
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: match col_index {
                                    0 => HorizontalAlign::Left,
                                    1 => HorizontalAlign::Center,
                                    2 => HorizontalAlign::Right,
                                    _ => HorizontalAlign::Left,
                                },
                            },
                        },
                        ..default()
                    })
                    .insert(Cell::new(row_index, col_index, is_label));
                }
            })
            .insert(Parent(screen))
            .insert(Line);
    }
}

/// Updates the UI of the MCDU screen with the data coming from the simulator
pub fn update_screen(
    mut q: Query<(&Cell, &mut Text)>,
    mut events: EventReader<ScreenUpdateEvent>,
    asset_server: Res<AssetServer>,
) {
    for screen_update_event in events.iter() {
        let screen_update = &screen_update_event.0;
        for (cell, mut text) in q.iter_mut() {
            let Cell {
                row_index,
                col_index,
                is_label,
            } = *cell;
            let parsed_text = &screen_update.lines[row_index][col_index];
            println!("{:?}", parsed_text);

            text.sections = build_text_sections(parsed_text, is_label, &asset_server);
        }
    }
}

/// Builds the text sections of a Text component given a tree-like representation of how
/// text should be segmented and styled
fn build_text_sections(
    parsed_text: &Graph<String, TextVertex, bool>,
    is_label: bool,
    asset_server: &Res<AssetServer>,
) -> Vec<TextSection> {
    let mut text_sections: Vec<TextSection> = Vec::new();

    for (vertex_id, edges) in parsed_text.adjacency.iter() {
        // Find leaves in the tree: a vertex/node is a leaf if it has a single edge that also
        // connects it to its parent
        let is_leaf = edges.len() == 1 && edges[0].1;
        if is_leaf {
            if let Some(vertex) = parsed_text.get_vertex(vertex_id.clone()) {
                // TODO: Handle AlignLeft, AlignRight, Space

                let font: Handle<Font> = asset_server.load(
                    if is_label || vertex.formatter == TextFormatter::FontSmall {
                        "HoneywellMCDUSmall.ttf"
                    } else {
                        "HoneywellMCDU.ttf"
                    },
                );
                let color: Color = match &vertex.formatter {
                    TextFormatter::ColorAmber => Color::rgb_u8(0xff, 0x9a, 0x00),
                    TextFormatter::ColorCyan => Color::rgb_u8(0x00, 0xff, 0xff),
                    TextFormatter::ColorGreen => Color::rgb_u8(0x00, 0xff, 0x00),
                    TextFormatter::ColorInop => Color::rgb_u8(0x66, 0x66, 0x66),
                    TextFormatter::ColorMagenta => Color::rgb_u8(0xff, 0x94, 0xff),
                    TextFormatter::ColorRed => Color::rgb_u8(0xff, 0x00, 0x00),
                    TextFormatter::ColorWhite => Color::rgb_u8(0xff, 0xff, 0xff),
                    TextFormatter::ColorYellow => Color::rgb_u8(0xff, 0xff, 0x00),
                    _ => Color::rgb_u8(0xff, 0xff, 0xff),
                };

                text_sections.push(TextSection {
                    value: vertex.value.clone().unwrap_or("".to_string()),
                    style: TextStyle {
                        font,
                        font_size: DEFAULT_HEIGHT * 0.055,
                        color,
                    },
                });
            }
        }
    }

    text_sections
}
