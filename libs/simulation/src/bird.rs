use rand::{Rng, RngCore};
use crate::eye;
use neural_network;
use neural_network::Network;
use crate::eye::Eye;

pub struct Bird {
    pub(crate) position: nalgebra::Point2<f32>,
    pub(crate) rotation: nalgebra::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: eye::Eye,
    pub(crate) brain: neural_network::Network,
    pub(crate) foods_eaten_count: usize
}

impl Bird {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Network::random(eye.get_cell_count());

        Self::new(eye, brain, rng)
    }

    pub fn new(eye: Eye, brain: Network, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.001,
            eye,
            brain,
            foods_eaten_count: 0
        }
    }

    pub fn get_position(&self) -> nalgebra::Point2<f32> {
        self.position
    }

    pub fn get_rotation(&self) -> nalgebra::Rotation2<f32> {
        self.rotation
    }

    pub fn get_chromosomes(&self) -> Vec<f32> {
        self.brain.get_weights().collect()
    }

    pub fn from_chromosome(chromosomes: Vec<f32>, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Network::from_chromosome(chromosomes, eye.get_cell_count());

        Self::new(eye, brain, rng)
    }
}