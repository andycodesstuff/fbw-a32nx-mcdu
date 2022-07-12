mod plugins;

use crate::plugins::{screen::ScreenPlugin, server::ServerPlugin};
use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::WorldInspectorPlugin;

pub const BG_COLOR: Color = Color::rgb(0.05, 0.08, 0.14);

// Describe the height and width of the MCDU screen in characters
pub const SCREEN_ROWS: usize = 14;
pub const SCREEN_COLS: usize = 25;

fn main() {
    let mut bevy_app = App::new();
    bevy_app
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(WindowDescriptor {
            title: "FlyByWire A32NX MCDU".to_string(),
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ScreenPlugin)
        .add_plugin(ServerPlugin)
        .add_startup_system(setup);

    if cfg!(feature = "debug-mode") {
        bevy_app.add_plugin(WorldInspectorPlugin::new());
    }

    bevy_app.run();
}

fn setup(mut commands: Commands) {
    // Setup 2D camera
    commands.spawn_bundle(UiCameraBundle::default());
}
