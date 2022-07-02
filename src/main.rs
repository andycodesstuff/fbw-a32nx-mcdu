mod plugins;

use crate::plugins::{screen::ScreenPlugin, server::ServerPlugin};
use bevy::prelude::*;

pub const ASPECT_RATIO: f32 = 5.0 / 4.0;
pub const DEFAULT_HEIGHT: f32 = 600.0;
pub const DEFAULT_WIDTH: f32 = DEFAULT_HEIGHT * ASPECT_RATIO;

// Describe the height and width of the MCDU screen in characters
pub const SCREEN_ROWS: usize = 14;
pub const SCREEN_COLS: usize = 24;
pub const SCREEN_PADDING: f32 = 8.0; // in pixels

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
        .add_plugin(ScreenPlugin)
        .add_plugin(ServerPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Setup 2D camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
