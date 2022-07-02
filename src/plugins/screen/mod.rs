pub mod components;
pub mod systems;

use crate::plugins::screen::{
    components::Scratchpad, components::Screen, systems::setup, systems::update_screen,
};
use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<Screen>()
            .register_inspectable::<Scratchpad>()
            .add_startup_system(setup)
            .add_system(update_screen);
    }
}
