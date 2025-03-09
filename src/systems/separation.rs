use crate::components::boid::Boid;
use bevy::prelude::*;

const SEPARATION_DISTANCE_THRESHOLD: f32 = 75.0;
const MAX_STEERING_ANGLE: f32 = 0.1;

pub fn boid_separation(mut query: Query<(&mut Boid, &Transform)>) {
    let mut combinations = query.iter_combinations_mut();

    while let Some([(mut boid, transform), (_, neighbor_transform)]) = combinations.fetch_next() {
        let distance = transform
            .translation
            .distance(neighbor_transform.translation);

        if distance < SEPARATION_DISTANCE_THRESHOLD {
            let separation_vector = (transform.translation - neighbor_transform.translation)
                .truncate()
                .normalize();

            boid.direction = boid
                .direction
                .rotate_towards(separation_vector, MAX_STEERING_ANGLE);
        }
    }
}
