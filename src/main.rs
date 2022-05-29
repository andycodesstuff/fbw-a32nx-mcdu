use bevy::prelude::*;

pub const ASPECT_RATIO: f32 = 4.0 / 3.0;
pub const DEFAULT_HEIGHT: f32 = 600.0;
pub const DEFAULT_WIDTH: f32 = DEFAULT_HEIGHT * ASPECT_RATIO;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            height: DEFAULT_HEIGHT,
            width: DEFAULT_WIDTH,
            resizable: false,
            title: "FlyByWire A32NX MCDU".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Setup 2D camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Render a horizontally and vertically aligned sample text
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "HELLO WORLD ____/____",
                    TextStyle {
                        font: asset_server.load("HoneywellMCDU.ttf"),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                    default(),
                ),
                ..default()
            });
        });
}
