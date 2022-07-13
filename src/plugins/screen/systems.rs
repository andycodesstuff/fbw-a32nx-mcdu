use super::{
    components::{Row, RowContent, RowFooter, RowHeader},
    systems_utils::{
        compute_font_size, compute_font_whitespace, compute_row_width, compute_text_bundles,
        TextAlign,
    },
};
use crate::{
    plugins::server::{ScreenUpdateEvent, TextSegment},
    SCREEN_ROWS,
};
use bevy::prelude::*;
use rand::Rng;

/// Set-ups the UI hierarchy
pub fn setup_system(mut commands: Commands, windows: Res<Windows>) {
    let mut rng = rand::thread_rng();

    let window = windows.get_primary().unwrap();
    let window_height = window.height();

    // Compute the width of the container element to show at most SCREEN_COLS characters of text
    let font_size = compute_font_size(window);
    let font_whitespace = compute_font_whitespace(font_size);
    let row_height = window_height / (SCREEN_ROWS as f32);
    let row_width = compute_row_width(font_size);

    // Root container
    #[rustfmt::skip]
    let root_color = if cfg!(feature = "debug-mode") { Color::TEAL } else { Color::NONE };
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::ColumnReverse,
                margin: Rect {
                    left: Val::Auto,
                    right: Val::Auto,
                    top: Val::Undefined,
                    bottom: Val::Undefined,
                },
                size: Size::new(Val::Px(row_width + font_whitespace), Val::Px(window_height)),
                ..default()
            },
            color: UiColor(root_color),
            ..default()
        })
        .id();

    // Screen rows
    for row_index in 0..SCREEN_ROWS {
        let is_label = row_index % 2 != 0;

        #[rustfmt::skip]
        let color_alpha = if cfg!(feature = "debug-mode") { 0.25 } else { 0.0 };
        let mut screen_row = commands.spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                padding: Rect {
                    left: Val::Px(font_whitespace),
                    right: Val::Undefined,
                    top: Val::Undefined,
                    bottom: Val::Undefined,
                },
                size: Size::new(Val::Percent(100.0), Val::Px(row_height)),
                ..default()
            },
            color: UiColor(Color::rgba(
                rng.gen_range(0.0..=1.0),
                rng.gen_range(0.0..=1.0),
                rng.gen_range(0.0..=1.0),
                color_alpha,
            )),
            ..default()
        });

        screen_row
            .insert(Row::new(row_index, is_label))
            .insert(Parent(root));

        if row_index == 0 {
            // Header row
            screen_row.insert(RowHeader);
        } else if row_index == SCREEN_ROWS - 1 {
            // Footer row
            screen_row.insert(RowFooter);
        } else {
            // Content rows
            screen_row.insert(RowContent);
        }
    }
}

/// Clears the screen before each update gets rendered
pub fn clear_screen_system(
    mut commands: Commands,
    mut events: EventReader<ScreenUpdateEvent>,
    rows_q: Query<Entity, With<Row>>,
) {
    events.iter().for_each(|_| {
        rows_q.for_each(|e| commands.entity(e).despawn_descendants());
    });
}

/// Updates the header section of the screen
pub fn update_header_row_system(
    mut commands: Commands,
    mut events: EventReader<ScreenUpdateEvent>,
    header_row_q: Query<Entity, (With<Row>, With<RowHeader>)>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    for screen_update_event in events.iter() {
        let header_row = header_row_q.get_single().unwrap();
        let window = windows.get_primary().unwrap();
        let screen_update = &screen_update_event.0;

        // Update the title of the current page
        let title = &screen_update.title;
        compute_text_bundles(title, TextAlign::Center, false, &asset_server, &window)
            .into_iter()
            .for_each(|b| {
                commands.spawn_bundle(b).insert(Parent(header_row));
            });

        // Update the left title
        let title_left = &screen_update.title_left;
        compute_text_bundles(title_left, TextAlign::Left, false, &asset_server, &window)
            .into_iter()
            .for_each(|b| {
                commands.spawn_bundle(b).insert(Parent(header_row));
            });

        // Update the page indicator
        let page = &screen_update.page;
        if !page.is_empty() {
            // Render the current page
            compute_text_bundles(page, TextAlign::Right, false, &asset_server, &window)
                .into_iter()
                .for_each(|b| {
                    commands.spawn_bundle(b).insert(Parent(header_row));
                });
        } else {
            // Render horizontal arrows instead
            let render_sx = screen_update.arrows[2];
            let render_dx = screen_update.arrows[3];
            let arrows = vec![
                TextSegment {
                    formatters: Vec::new(),
                    value: (if render_sx { "←" } else { "" }).to_string(),
                },
                TextSegment {
                    formatters: Vec::new(),
                    value: (if render_dx { "→" } else { "" }).to_string(),
                },
            ];

            compute_text_bundles(&arrows, TextAlign::Right, false, &asset_server, &window)
                .into_iter()
                .for_each(|b| {
                    commands.spawn_bundle(b).insert(Parent(header_row));
                });
        }
    }
}

/// Updates the header section of the screen
pub fn update_footer_row_system(
    mut commands: Commands,
    mut events: EventReader<ScreenUpdateEvent>,
    footer_row_q: Query<Entity, (With<Row>, With<RowFooter>)>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    for screen_update_event in events.iter() {
        let footer_row = footer_row_q.get_single().unwrap();
        let window = windows.get_primary().unwrap();
        let screen_update = &screen_update_event.0;

        // Update the scratchpad
        let scratchpad = &screen_update.scratchpad;
        compute_text_bundles(scratchpad, TextAlign::Left, false, &asset_server, &window)
            .into_iter()
            .for_each(|b| {
                commands.spawn_bundle(b).insert(Parent(footer_row));
            });

        // Update the vertical scroll indicator
        let render_up = screen_update.arrows[0];
        let render_down = screen_update.arrows[1];
        let arrows = vec![
            TextSegment {
                formatters: Vec::new(),
                value: (if render_down { "↓" } else { "" }).to_string(),
            },
            TextSegment {
                formatters: Vec::new(),
                value: (if render_up { "↑" } else { "" }).to_string(),
            },
        ];

        compute_text_bundles(&arrows, TextAlign::Right, false, &asset_server, &window)
            .into_iter()
            .for_each(|b| {
                commands.spawn_bundle(b).insert(Parent(footer_row));
            });
    }
}
