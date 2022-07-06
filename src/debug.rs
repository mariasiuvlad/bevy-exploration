use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            println!("Adding debug plugin...");
            app.insert_resource(WorldInspectorParams {
                despawnable_entities: true,
                highlight_changes: true,
                enabled: true,
                ..Default::default()
            })
            .add_plugin(WorldInspectorPlugin::new());
        }
    }
}
