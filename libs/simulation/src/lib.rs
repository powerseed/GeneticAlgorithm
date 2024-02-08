use rand::{Rng, RngCore};

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
pub struct World {
    birds: Vec<Bird>,
    foods: Vec<Food>
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

pub struct Bird {
    position: nalgebra::Point2<f32>,
    rotation: nalgebra::Rotation2<f32>,
    speed: f32,
}

impl Bird {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.001
        }
    }

    pub fn get_position(&self) -> nalgebra::Point2<f32> {
        self.position
    }

    pub fn get_rotation(&self) -> nalgebra::Rotation2<f32> {
        self.rotation
    }
}

pub struct Food {
    position: nalgebra::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen()
        }
    }
    pub fn get_position(&self) -> nalgebra::Point2<f32> {
        self.position
    }
}