use bevy::prelude::*;
use bevy_inspector_egui::{
    widgets::InNewWindow, Inspectable, InspectorPlugin, WorldInspectorParams, WorldInspectorPlugin,
};

#[derive(Inspectable, Default)]
struct SomeComplexType {
    very_long_field_name: Color,
}

#[derive(Inspectable, Default)]
struct Inspector {
    a: f32,
    #[inspectable(title = "Complex Type", resizable)]
    window: InNewWindow<SomeComplexType>,
}

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
            .add_plugin(InspectorPlugin::<Inspector>::new())
            .add_plugin(WorldInspectorPlugin::new());
        }
    }
}
