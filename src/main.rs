use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "FlyByWire A32NX MCDU".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
