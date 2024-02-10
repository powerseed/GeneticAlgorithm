pub mod world;
pub mod food;
pub mod bird;
mod eye;

use nalgebra::wrap;
use rand::{Rng, RngCore};
use crate::world::World;

const SPEED_MAX: f32 = 0.003;
const SPEED_MIN:f32 = 0.001;
const SPEED_ACCELERATION_MAX:f32 = 0.0005;
const ROTATION_ACCELERATION_MAX:f32 = std::f32::consts::FRAC_PI_2;

pub struct Simulation {
    world: World
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng)
        }
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        // process movements
        for bird in &mut self.world.birds {
            bird.position += bird.rotation * nalgebra::Vector2::new(0.0, bird.speed);
            bird.position.x = nalgebra::wrap(bird.position.x, 0.0, 1.0);
            bird.position.y = nalgebra::wrap(bird.position.y, 0.0, 1.0);
        }

        // process collisions
        for bird in &mut self.world.birds {
            for food in &mut self.world.foods {
                let distance = nalgebra::distance(&bird.position, &food.position);
                if distance < 0.02 {
                    food.position = rng.gen();
                }
            }
        }

        // process brains
        for bird in &mut self.world.birds {
            let foods_seen_by_eye_cells = bird.eye.see_foods(bird.position, bird.rotation, &self.world.foods);
            let speed_and_rotation_delta = bird.brain.propagate(foods_seen_by_eye_cells);
            let speed_delta = speed_and_rotation_delta[0].clamp(-SPEED_ACCELERATION_MAX, SPEED_ACCELERATION_MAX);
            let rotation_delta = speed_and_rotation_delta[1].clamp(-ROTATION_ACCELERATION_MAX, ROTATION_ACCELERATION_MAX);

            bird.speed = (bird.speed + speed_delta).clamp(SPEED_MIN, SPEED_MAX);
            bird.rotation = nalgebra::Rotation2::new(bird.rotation.angle() + rotation_delta);
        }
    }
}