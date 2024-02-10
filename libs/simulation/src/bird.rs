use rand::{Rng, RngCore};
use crate::eye;
use neural_network;
pub struct Bird {
    pub(crate) position: nalgebra::Point2<f32>,
    pub(crate) rotation: nalgebra::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: eye::Eye,
    pub(crate) brain: neural_network::Network
}

impl Bird {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = eye::Eye::default();
        let eye_cell_count = eye.get_cell_count();
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.001,
            eye,
            brain: neural_network::Network::random(vec![eye_cell_count, 2 * eye_cell_count, 2])
        }
    }

    pub fn get_position(&self) -> nalgebra::Point2<f32> {
        self.position
    }

    pub fn get_rotation(&self) -> nalgebra::Rotation2<f32> {
        self.rotation
    }
}