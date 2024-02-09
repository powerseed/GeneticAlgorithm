pub mod world;
pub mod food;
pub mod bird;
mod eye;

use rand::{Rng, RngCore};
use crate::world::World;

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
        for bird in &mut self.world.birds {
            bird.position += bird.rotation * nalgebra::Vector2::new(0.0, bird.speed);
            bird.position.x = nalgebra::wrap(bird.position.x, 0.0, 1.0);
            bird.position.y = nalgebra::wrap(bird.position.y, 0.0, 1.0);
        }

        for bird in &mut self.world.birds {
            for food in &mut self.world.foods {
                let distance = nalgebra::distance(&bird.position, &food.position);
                if distance < 0.02 {
                    food.position = rng.gen();
                }
            }
        }
    }
}