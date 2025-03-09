use bevy::prelude::*;

#[derive(Component)]
pub struct Boid {
    pub velocity: Vec3,
    pub visual_range: f32,
    pub protected_range: f32,
    pub centering_factor: f32,
    pub avoid_factor: f32,
    pub matching_factor: f32,
    pub min_speed: f32,
    pub max_speed: f32,
}

impl Default for Boid {
    fn default() -> Self {
        Boid {
            velocity: Vec3::new(
                rand::random_range(-1.0..1.0),
                rand::random_range(-1.0..1.0),
                0.0,
            )
            .normalize(),
            visual_range: 40.0,
            protected_range: 24.0,
            centering_factor: 0.0005,
            avoid_factor: 0.05,
            matching_factor: 0.05,
            min_speed: 150.0,
            max_speed: 200.0,
        }
    }
}
