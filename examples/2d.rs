use bevy::prelude::*;
use bevy_boids::{BoidsPlugin, components::boid::Boid};

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
    let boid_amount = 100;
    let boid_size = 7.0;

    commands.spawn(Camera2d);

    let window = window.single();
    let width = window.resolution.width() / 2.0 - boid_size;
    let height = window.resolution.height() / 2.0 - boid_size;

    for _ in 0..=boid_amount {
        commands.spawn((
            Mesh2d(meshes.add(Triangle2d::new(
                Vec2::Y * boid_size * 2.5,
                Vec2::new(-boid_size, -boid_size),
                Vec2::new(boid_size, -boid_size),
            ))),
            MeshMaterial2d(materials.add(Color::hsl(217.0, 0.7, rand::random_range(0.4..=0.7)))),
            Transform::from_xyz(
                rand::random_range(-width..=width),
                rand::random_range(-height..=height),
                0.0,
            ),
            Boid {
                velocity: Vec3::new(
                    rand::random_range(-1.0..1.0),
                    rand::random_range(-1.0..1.0),
                    0.0,
                )
                .normalize(),
                ..Boid::default()
            },
        ));
    }
}
