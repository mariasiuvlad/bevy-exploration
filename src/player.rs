use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::prelude::*;

use crate::{enemy::Enemy, magic_missiles::MagicMisslesEvent};

#[derive(Component, Inspectable)]
pub struct Player;

#[derive(Component, Inspectable)]
pub struct MovementSpeed(f32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(player_actions);
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Name::new("Player"))
        .insert(MovementSpeed(10.0))
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(Velocity {
            linvel: Vec3::splat(0.0),
            angvel: Vec3::splat(0.0),
        })
        .insert(Restitution::coefficient(0.7))
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(CollisionGroups::new(0b1000, 0b1100))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .with_children(|parent| {
            parent.spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_xyz(-16.0, 16.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            });
        });
}

fn player_movement(
    mut commands: Commands,
    mut player: Query<(Entity, &MovementSpeed, &mut Velocity), With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player_entity, movement_speed, mut _velocity) = player.get_single_mut().unwrap();

    let ms = movement_speed.0 * time.delta_seconds() * 20.0;
    if keyboard.just_pressed(KeyCode::W) {
        commands.entity(player_entity).insert(ExternalImpulse {
            impulse: Vec3::new(ms, 0.0, 0.0),
            torque_impulse: Vec3::ZERO,
        });
        // velocity.linvel += Vec3::new(ms, 0.0, 0.0);
    }
    if keyboard.just_pressed(KeyCode::A) {
        commands.entity(player_entity).insert(ExternalImpulse {
            impulse: Vec3::new(0.0, 0.0, ms),
            torque_impulse: Vec3::ZERO,
        });
        // velocity.linvel += Vec3::new(0.0, 0.0, ms);
    }
    if keyboard.just_pressed(KeyCode::S) {
        commands.entity(player_entity).insert(ExternalImpulse {
            impulse: Vec3::new(-ms, 0.0, 0.0),
            torque_impulse: Vec3::ZERO,
        });
        // velocity.linvel += Vec3::new(-ms, 0.0, 0.0);
    }
    if keyboard.just_pressed(KeyCode::D) {
        commands.entity(player_entity).insert(ExternalImpulse {
            impulse: Vec3::new(0.0, 0.0, -ms),
            torque_impulse: Vec3::ZERO,
        });
        // velocity.linvel += Vec3::new(0.0, 0.0, -ms);
    }
    if keyboard.just_pressed(KeyCode::Space) {
        commands.entity(player_entity).insert(ExternalImpulse {
            impulse: Vec3::new(0.0, ms * 20.0, 0.0),
            torque_impulse: Vec3::ZERO,
        });
        // velocity.linvel += Vec3::new(0.0, ms * 20.0, 0.0);
    }
}

fn player_actions(
    mut ev_mm: EventWriter<MagicMisslesEvent>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (player_entity, transform) = player_query.get_single().unwrap();
    let enemy_entity = enemy_query.get_single().unwrap();
    if keyboard.just_pressed(KeyCode::X) {
        ev_mm.send(MagicMisslesEvent {
            source: player_entity,
            target: enemy_entity,
            origin: transform.translation,
        });
    }
}
