use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy);
    }
}

#[derive(Component)]
pub struct Enemy;

fn spawn_enemy(mut commands: Commands) {
    commands
        .spawn()
        .insert(Enemy)
        .insert(Name::new("Enemy"))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.0, 2.0, 1.0))
        .insert(CollisionGroups::new(0b1000, 0b1100))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(Velocity {
            linvel: Vec3::splat(0.0),
            angvel: Vec3::splat(0.0),
        })
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -10.0)));
}
