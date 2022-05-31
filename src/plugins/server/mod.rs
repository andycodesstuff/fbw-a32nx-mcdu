pub mod systems;

use crate::plugins::server::systems::setup;
use bevy::prelude::*;
use serde::Deserialize;

/// Represents an update that has to be drawn on the MCDU screen
#[derive(Debug, Deserialize)]
pub struct ScreenUpdate {
    pub lines: Vec<Vec<String>>,
    pub scratchpad: String,
    pub title: String,
    #[serde(alias = "titleLeft")]
    pub title_left: String,
    pub arrows: Vec<bool>,
}

/// Represents the event associated with a screen update request
pub struct ScreenUpdateEvent(pub ScreenUpdate);

/// Represents the message sent to the server when a screen update is requested by the client
#[derive(Debug, Deserialize)]
struct ScreenUpdateMessage {
    right: ScreenUpdate,
    left: ScreenUpdate,
}

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScreenUpdateEvent>()
            .add_startup_system(setup);
    }
}
