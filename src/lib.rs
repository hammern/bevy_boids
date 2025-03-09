use bevy::prelude::*;
use systems::{
    alignment::boid_alignment, cohesion::boid_cohesion, movement::boid_movement,
    separation::boid_separation,
};

pub mod components;
mod systems;

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                boid_separation,
                boid_alignment,
                boid_cohesion,
                boid_movement,
            )
                .chain(),
        );
    }
}
