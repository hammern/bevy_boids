use bevy::prelude::*;
use bevy_boids::{BoidsPlugin, components::boid::Boid};

const BOID_AMOUNT: u8 = 50;
const BOID_SIZE: f32 = 7.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BoidsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    commands.spawn(Camera2d);

    let window = window.single();
    let width = window.resolution.width() / 2.0 - BOID_SIZE;
    let height = window.resolution.height() / 2.0 - BOID_SIZE;

    for _ in 0..=BOID_AMOUNT {
        commands.spawn((
            Mesh2d(meshes.add(Triangle2d::new(
                Vec2::Y * BOID_SIZE * 2.5,
                Vec2::new(-BOID_SIZE, -BOID_SIZE),
                Vec2::new(BOID_SIZE, -BOID_SIZE),
            ))),
            MeshMaterial2d(materials.add(Color::hsl(217.0, 0.7, rand::random_range(0.4..=0.7)))),
            Transform::from_xyz(
                rand::random_range(-width..=width),
                rand::random_range(-height..=height),
                0.0,
            ),
            Boid::default(),
        ));
    }
}
