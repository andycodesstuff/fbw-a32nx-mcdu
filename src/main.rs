mod plugins;

use crate::plugins::{screen::ScreenPlugin, server::ServerPlugin};
use bevy::{prelude::*, window::WindowMode};

pub const BG_COLOR: Color = Color::rgb(0.05, 0.08, 0.14);

// Describe the height and width of the MCDU screen in characters
pub const SCREEN_ROWS: usize = 14;
pub const SCREEN_COLS: usize = 25;

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(WindowDescriptor {
            title: "FlyByWire A32NX MCDU".to_string(),
            mode: WindowMode::BorderlessFullscreen,
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
    commands.spawn_bundle(UiCameraBundle::default());
}
