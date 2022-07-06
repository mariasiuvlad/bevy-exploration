use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::prelude::*;

#[derive(Component, Inspectable)]
pub struct Shield;

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_shield);
    }
}

pub fn spawn_shield(mut commands: Commands) {
    commands
        .spawn()
        .insert(Shield)
        .insert(Name::new("Shield"))
        .insert(CollisionGroups::new(0b0100, 0b0100))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(4.0, 4.0, 0.5))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -4.0)));
}
