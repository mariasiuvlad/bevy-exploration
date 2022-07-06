use std::ops::Add;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::prelude::*;

#[derive(Component, Inspectable)]
pub struct MagicMissle {
    target: Vec3,
    follow: Entity,
}

pub struct MagicMisslesEvent {
    pub source: Entity,
    pub target: Entity,
    pub origin: Vec3,
}

#[derive(Component, Inspectable)]
pub struct Contact(Entity);

pub struct MagicMisslesPlugin;

impl Plugin for MagicMisslesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MagicMisslesEvent>()
            .add_system(handle_magic_missiles)
            .add_system(magic_missile_tracking.before(magic_missile_contact))
            .add_system(resolve_collision_events)
            .add_system(magic_missile_contact);
    }
}

pub fn handle_magic_missiles(mut commands: Commands, mut ev_mm: EventReader<MagicMisslesEvent>) {
    for ev in ev_mm.iter() {
        spawn_magic_missiles(&mut commands, ev);
    }
}

fn spawn_magic_missiles(commands: &mut Commands, e: &MagicMisslesEvent) {
    commands
        .spawn()
        .insert(MagicMissle {
            target: Vec3::new(0.0, 0.0, -10.0),
            follow: e.target,
        })
        .insert(Name::new("Magic Missle"))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.6))
        .insert(CollisionGroups::new(0b0100, 0b1100))
        .insert(ColliderMassProperties::Density(1.0))
        .insert(GravityScale(0.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ExternalForce::default())
        .insert_bundle(TransformBundle::from(Transform::from_translation(
            e.origin.add(Vec3::new(3.0, 0.0, 0.0)),
        )));
}

fn magic_missile_tracking(
    mut mm_query: Query<(&mut MagicMissle, &mut Transform, &mut ExternalForce), With<MagicMissle>>,
    target_query: Query<&Transform, Without<MagicMissle>>,
) {
    for (mut mm, mut mm_transform, mut external_force) in mm_query.iter_mut() {
        match target_query.get(mm.follow) {
            Ok(t) => {
                mm.target = t.translation;
            }
            Err(e) => {
                eprintln!("QueryEntityError {}", e);
            }
        }
        mm_transform.look_at(mm.target, Vec3::Y);
        external_force.force = mm_transform.forward() * 6.0;
    }
}

fn resolve_collision_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    // mut mm_query: Query<Entity, With<MagicMissle>>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(a, b, _) => {
                commands.entity(*a).insert(Contact(*b));
                commands.entity(*b).insert(Contact(*a));
            }
            CollisionEvent::Stopped(a, b, c) => {}
        }
    }
}

fn magic_missile_contact(
    mut commands: Commands,
    mm_query: Query<(Entity, &Contact, &MagicMissle)>,
) {
    for (e, c, mm) in mm_query.iter() {
        if mm.follow == c.0 {
            println!("On hit!");
            commands.entity(e).despawn();
        }
    }
}
