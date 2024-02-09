use rand::RngCore;
use crate::bird::Bird;
use crate::food::Food;

pub struct World {
    pub(crate) birds: Vec<Bird>,
    pub(crate) foods: Vec<Food>
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            birds: (0..10).map(|_| Bird::random(rng)).collect(),
            foods: (0..30).map(|_| Food::random(rng)).collect()
        }
    }

    pub fn get_birds(&self) -> &[Bird] {
        &self.birds
    }

    pub fn get_foods(&self) -> &[Food] {
        &self.foods
    }
}