use crate::components::boid::Boid;
use bevy::prelude::*;

pub fn boid_movement(
    mut query: Query<(&Boid, &mut Transform)>,
    time: Res<Time>,
    window: Query<&Window>,
) {
    let window = window.single();
    let width = window.resolution.width() / 2.0;
    let height = window.resolution.height() / 2.0;

    for (boid, mut transform) in &mut query {
        transform.rotation = Quat::from_rotation_z(-boid.direction.x.atan2(boid.direction.y));

        transform.translation += Vec3::new(boid.direction.x, boid.direction.y, 0.0)
            * boid.movement_speed
            * time.delta_secs();

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
