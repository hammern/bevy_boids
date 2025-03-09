use crate::components::boid::Boid;
use bevy::prelude::*;

pub fn boid_simulation(
    mut query: Query<(&mut Boid, &mut Transform)>,
    time: Res<Time>,
    window: Query<&Window>,
) {
    let mut calculations: Vec<(Vec3, Vec3, Vec3, f32)> = vec![];

    for (boid, transform) in &query {
        let mut close_position = Vec3::ZERO;
        let mut average_position = Vec3::ZERO;
        let mut average_velocity = Vec3::ZERO;
        let mut neighboring_boids = 0.0;

        for (neighbor_boid, neighbor_transform) in &query {
            let distance = transform
                .translation
                .distance(neighbor_transform.translation);

            if distance.abs() < boid.visual_range {
                let squared_distance = distance.powi(2);

                if squared_distance < boid.protected_range.powi(2) {
                    close_position += transform.translation - neighbor_transform.translation;
                } else if squared_distance < boid.visual_range.powi(2) {
                    average_position += neighbor_transform.translation;
                    average_velocity += neighbor_boid.velocity;
                    neighboring_boids += 1.0;
                }
            }
        }

        if neighboring_boids > 0.0 {
            average_position /= neighboring_boids;
            average_velocity /= neighboring_boids;
        }

        calculations.push((
            close_position,
            average_position,
            average_velocity,
            neighboring_boids,
        ));
    }

    let window = window.single();
    let width = window.resolution.width() / 2.0;
    let height = window.resolution.height() / 2.0;

    let mut i = 0;
    for (mut boid, mut transform) in &mut query {
        let (close_position, average_position, average_velocity, neighboring_boids) =
            calculations[i];
        i += 1;

        if neighboring_boids > 0.0 {
            boid.velocity.x = boid.velocity.x
                + (average_position.x - transform.translation.x) * boid.centering_factor
                + (average_velocity.x - boid.velocity.x) * boid.matching_factor;
            boid.velocity.y = boid.velocity.y
                + (average_position.y - transform.translation.y) * boid.centering_factor
                + (average_velocity.y - boid.velocity.y) * boid.matching_factor;
        }

        let avoid_factor = boid.avoid_factor;
        boid.velocity += close_position * avoid_factor;

        let speed = (boid.velocity.x.powi(2) + boid.velocity.y.powi(2)).sqrt();

        if speed < boid.min_speed {
            boid.velocity = (boid.velocity / speed) * boid.min_speed;
        } else if speed > boid.max_speed {
            boid.velocity = (boid.velocity / speed) * boid.max_speed;
        }

        transform.rotation = Quat::from_rotation_z(-boid.velocity.x.atan2(boid.velocity.y));
        transform.translation += boid.velocity * time.delta_secs();

        if transform.translation.x > width {
            transform.translation.x = -width;
        }
        if transform.translation.x < -width {
            transform.translation.x = width;
        }
        if transform.translation.y > height {
            transform.translation.y = -height;
        }
        if transform.translation.y < -height {
            transform.translation.y = height;
        }
    }
}
