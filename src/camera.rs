use bevy::prelude::*;

pub struct CameraBundle {}

fn setup(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-8.0, 8.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
