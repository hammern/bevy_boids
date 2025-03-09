use crate::components::boid::Boid;
use bevy::prelude::*;

const MAX_STEERING_ANGLE: f32 = 0.1;

pub fn boid_cohesion(mut query: Query<(&mut Boid, &Transform)>) {
    let mut center_of_mass = Vec2::ZERO;
    let mut boid_count = 0.0;

    for (_, transform) in &query {
        center_of_mass += transform.translation.truncate();
        boid_count += 1.0;
    }
    center_of_mass /= boid_count;

    for (mut boid, transform) in &mut query {
        let cohesion_vector = (center_of_mass - transform.translation.truncate()).normalize();

        boid.direction = boid
            .direction
            .rotate_towards(cohesion_vector, MAX_STEERING_ANGLE);
    }
}
