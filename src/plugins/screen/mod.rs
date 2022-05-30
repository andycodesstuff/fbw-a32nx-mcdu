pub mod components;
pub mod systems;

pub const SCREEN_LINES: usize = 12;
pub const SCREEN_COLUMNS: usize = 3;
pub const SCREEN_PADDING: f32 = 4.2; // in percents

use crate::plugins::screen::{
    components::Cell, components::Line, components::Screen, systems::screen_update, systems::setup,
};
use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_inspectable::<Cell>()
            .register_inspectable::<Line>()
            .register_inspectable::<Screen>()
            .add_startup_system(setup)
            .add_system(screen_update);
    }
}
