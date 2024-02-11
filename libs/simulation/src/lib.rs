pub mod world;
pub mod food;
pub mod bird;
mod eye;
mod bird_individual;

use rand::{Rng, RngCore};
use crate::world::World;
use genetic_algorithm;
use crate::bird_individual::BirdIndividual;

const SPEED_MAX: f32 = 0.005;
const SPEED_MIN:f32 = 0.001;
const SPEED_ACCELERATION_MAX:f32 = 0.001;
const ROTATION_ACCELERATION_MAX:f32 = std::f32::consts::FRAC_PI_2;
const BIRD_LIFESPAN_IN_STEP: usize = 2500;

pub struct Simulation {
    world: World,
    genetic_algorithm: genetic_algorithm::GeneticAlgorithm,
    steps_completed: usize
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
            genetic_algorithm: genetic_algorithm::GeneticAlgorithm::new(),
            steps_completed: 0
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
                    bird.foods_eaten_count += 1;
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

        self.steps_completed += 1;

        // generate the next generation.
        if self.steps_completed > BIRD_LIFESPAN_IN_STEP {
           self.evolve(rng);
        }
    }

    pub fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.steps_completed = 0;

        let bird_individuals: Vec<BirdIndividual> = self.world.birds
            .iter()
            .map(BirdIndividual::from_bird)
            .collect();

        let bird_individuals = self.genetic_algorithm.evolve(rng, &bird_individuals);

        self.world.birds = bird_individuals
            .into_iter()
            .map(|bird_individual| bird_individual.to_bird(rng))
            .collect();
    }
}