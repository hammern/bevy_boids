use bevy::prelude::*;
use systems::movement::boid_movement;

pub mod components;
mod systems;

pub struct BoidsPlugin;

impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, boid_movement);
    }
}
