use crate::plugins::screen::{
    components::Line, components::Screen, SCREEN_COLUMNS, SCREEN_LINES, SCREEN_PADDING,
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
    for i in 0..SCREEN_LINES {
        let is_label = i % 2 == 0;

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    flex_grow: 1.0,
                    ..default()
                },
                color: UiColor(if is_label { Color::BLACK } else { Color::WHITE }),
                ..default()
            })
            .with_children(|line| {
                // Line columns
                for _ in 0..SCREEN_COLUMNS {
                    line.spawn_bundle(TextBundle::default());
                }
            })
            .insert(Parent(screen))
            .insert(Line {
                is_label,
                ..default()
            });
    }
}

/// Updates the MCDU's screen with the data coming from the simulator
pub fn screen_update(
    screen_q: Query<&Screen>,
    mut lines_q: Query<(Entity, &Children), With<Line>>,
    mut columns_q: Query<&mut Text>,
) {
    for _ in screen_q.iter() {
        for (_, columns) in lines_q.iter_mut() {
            for column in columns.iter() {
                if let Ok(text) = columns_q.get_mut(*column) {
                    if text.sections.len() == 0 {
                        continue;
                    }

                    // Update columns here
                }
            }
        }
    }
}

/// DEBUG ONLY: Updates the lines' background based on whether the line is a label or not
#[allow(non_snake_case)]
pub fn DEBUG_lines_label_update(mut lines_q: Query<(&Line, &mut UiColor)>) {
    for (line, mut color) in lines_q.iter_mut() {
        color.0 = if line.is_label {
            Color::BLACK
        } else {
            Color::WHITE
        };
    }
}
