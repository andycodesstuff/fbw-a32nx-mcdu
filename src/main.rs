mod plugins;
mod utils;

use crate::plugins::{screen::ScreenPlugin, server::ServerPlugin};
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
