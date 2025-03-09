use bevy::prelude::*;

const BOID_MOVEMENT_SPEED: f32 = 150.0;

#[derive(Component)]
pub struct Boid {
    pub movement_speed: f32,
    pub direction: Vec2,
}

impl Default for Boid {
    fn default() -> Self {
        Boid {
            movement_speed: BOID_MOVEMENT_SPEED,
            direction: Vec2::new(rand::random(), rand::random()).normalize(),
        }
    }
}
