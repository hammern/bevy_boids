use crate::components::boid::Boid;
use bevy::prelude::*;

const MAX_STEERING_ANGLE: f32 = 0.1;

pub fn boid_alignment(mut query: Query<&mut Boid>) {
    let mut average_velocity = Vec2::ZERO;
    let mut boid_count = 0.0;

    for boid in &query {
        average_velocity += boid.direction;
        boid_count += 1.0;
    }
    average_velocity /= boid_count;

    for mut boid in &mut query {
        let alignment_vector = (average_velocity - boid.direction).normalize();

        boid.direction = boid
            .direction
            .rotate_towards(alignment_vector, MAX_STEERING_ANGLE);
    }
}
