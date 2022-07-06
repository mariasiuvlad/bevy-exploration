use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::{prelude::*, rapier::prelude::CollisionEventFlags};

/// Component which will be filled (if present) with a list of entities with which the current entity is currently in contact.
#[derive(Component, Default)]
pub struct Collisions(HashMap<Entity, CollisionEventFlags>);

impl Collisions {
    /// Returns the number of colliding entities.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there is no colliding entities.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the collisions contains the specified entity.
    pub fn contains(&self, entity: &Entity) -> bool {
        self.0.contains_key(entity)
    }

    /// An iterator visiting all colliding entities in arbitrary order.
    pub fn entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.0.keys().copied()
    }

    /// An iterator visiting all data from colliding entities in arbitrary order.
    pub fn collision_data(&self) -> impl Iterator<Item = &CollisionEventFlags> + '_ {
        self.0.values()
    }
}

/// Adds entity to [`CollidingEntities`] on starting collision and removes from it when the
/// collision end.
pub fn update_collisions(
    mut collision_events: EventReader<'_, '_, CollisionEvent>,
    mut collisions: Query<'_, '_, &mut Collisions>,
) {
    for event in collision_events.iter() {
        // let (data1, data2) = event.clone().data();
        match event {
            CollisionEvent::Started(e1, e2, flags) => {
                if let Ok(mut entities) = collisions.get_mut(*e1) {
                    entities.0.insert(*e2, *flags);
                }
                if let Ok(mut entities) = collisions.get_mut(*e2) {
                    entities.0.insert(*e2, *flags);
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if let Ok(mut entities) = collisions.get_mut(*e1) {
                    entities.0.remove(e2);
                }
                if let Ok(mut entities) = collisions.get_mut(*e2) {
                    entities.0.remove(e1);
                }
            }
        }
    }
}
