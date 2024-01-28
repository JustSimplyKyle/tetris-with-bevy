use bevy::{prelude::*, utils::HashMap};

use crate::SIZE;

#[derive(Component, Debug)]
pub struct Collider {
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new() -> Self {
        Self {
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities = HashMap::new();

    for [(entity_a, transform_a, _), (entity_b, transform_b, _)] in
        query.iter_combinations().filter(|[x, y]| x.0 != y.0)
    {
        let distance = transform_a
            .translation()
            .distance(transform_b.translation());
        if distance < SIZE {
            colliding_entities
                .entry(entity_a)
                .or_insert_with(Vec::new)
                .push(entity_b);
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.colliding_entities.extend_from_slice(&collisions)
        }
    }
}
