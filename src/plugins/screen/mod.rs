pub mod components;
pub mod systems;

use bevy::prelude::*;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, _app: &mut App) {}
}
