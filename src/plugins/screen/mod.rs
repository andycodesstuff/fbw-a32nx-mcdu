pub mod components;
pub mod systems;

use crate::plugins::screen::{
    components::Screen,
    systems::{setup, update_screen, update_screen_header},
};
use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<Screen>()
            .add_startup_system(setup)
            .add_system(update_screen_header)
            .add_system(update_screen);
    }
}
