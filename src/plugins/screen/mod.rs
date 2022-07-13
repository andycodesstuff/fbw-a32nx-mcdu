pub mod components;
pub mod systems;
pub(self) mod systems_utils;

use self::systems::{
    clear_screen_system, setup_system, update_footer_row_system, update_header_row_system,
};
use bevy::prelude::*;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct ClearScreen;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
struct UpdateScreen;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(clear_screen_system.label(ClearScreen).before(UpdateScreen))
            .add_system_set(
                SystemSet::new()
                    .label(UpdateScreen)
                    .after(ClearScreen)
                    .with_system(update_header_row_system)
                    .with_system(update_footer_row_system),
            );
    }
}
