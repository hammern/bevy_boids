# bevy_boids

A simulation of a flock of birds in bevy. This project is inspired by the boids coding adventure [video](https://www.youtube.com/watch?v=bqtqltqcQhw) by [Sebastian Lague](https://www.youtube.com/@SebastianLague).

Boids follow three main rules:

1. Separation - steer away from nearby boids to avoid crashing into them
2. Alignment - steer to move in the same direction as nearby boids
3. Cohesion - steer towards the center of any nearby boids

## Bevy versions

| `bevy_boids`   | `bevy`    |
|----------------|-----------|
| 0.1.x          | 0.15.x    |

## Examples

### 2D boids

Run `cargo run --example 2d`
