use crate::plugins::{
    screen::{
        components::Cell, components::Line, components::Screen, SCREEN_COLUMNS, SCREEN_LINES,
        SCREEN_PADDING,
    },
    server::ScreenUpdateEvent,
};
use bevy::prelude::*;

/// Set-ups the UI hierarchy of the MCDU and the elements that will be populated with data
pub fn setup(mut commands: Commands) {
    // Root container
    let screen = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
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
                    align_items: AlignItems::Center,
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
                    line.spawn_bundle(TextBundle::default())
                        .insert(Cell::new(row_index, col_index, is_label));
                }
            })
            .insert(Parent(screen))
            .insert(Line);
    }
}

/// Updates the MCDU's screen with the data coming from the simulator
pub fn screen_update(
    mut cells_q: Query<(&Cell, &mut Text)>,
    mut events: EventReader<ScreenUpdateEvent>,
) {
    for screen_update_event in events.iter() {
        let _screen_update = &screen_update_event.0;
        for (_cell, mut _text) in cells_q.iter_mut() {
            // Update cells here
        }
    }
}
