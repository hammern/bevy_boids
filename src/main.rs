use bevy::prelude::*;

const BOID_AMOUNT: u8 = 50;
const BOID_SIZE: f32 = 7.0;
const BOID_MOVEMENT_SPEED: f32 = 150.0;

#[derive(Component)]
struct Boid {
    movement_speed: f32,
    direction: Vec2,
}

impl Default for Boid {
    fn default() -> Self {
        Boid {
            movement_speed: BOID_MOVEMENT_SPEED,
            direction: Vec2::new(rand::random(), rand::random()).normalize(),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, boid_movement)
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

fn boid_movement(
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
