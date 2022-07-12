pub mod components;
pub mod systems;

use crate::plugins::screen::systems::{
    setup, update_screen_footer, update_screen_header, update_screen_main_content,
};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_startup_system(setup)
            .add_system(update_screen_header)
            .add_system(update_screen_main_content)
            .add_system(update_screen_footer);
    }
}
