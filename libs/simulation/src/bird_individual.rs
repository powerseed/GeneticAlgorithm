use rand::{RngCore};
use genetic_algorithm::Individual;
use crate::bird::Bird;

pub struct BirdIndividual {
    fitness: f32,
    chromosomes: Vec<f32>
}
impl Individual for BirdIndividual {
    fn new(chromosomes: Vec<f32>) -> Self {
        Self {
            fitness: 0.0,
            chromosomes
        }
    }
    fn fitness(&self) -> f32 {
        self.fitness
    }
    fn chromosomes(&self) -> &Vec<f32> {
        &self.chromosomes
    }
}

impl BirdIndividual {
    pub fn from_bird(bird: &Bird) -> Self {
        Self {
            fitness: bird.foods_eaten_count as f32,
            chromosomes: bird.get_chromosomes()
        }
    }

    pub fn to_bird(self, rng: &mut dyn RngCore) -> Bird {
        Bird::from_chromosome(self.chromosomes, rng)
    }
}