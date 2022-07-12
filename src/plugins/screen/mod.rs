pub mod components;
pub mod systems;
pub(self) mod systems_utils;

use self::systems::{clear_screen_system, setup_system};
use bevy::prelude::*;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(clear_screen_system);
    }
}
