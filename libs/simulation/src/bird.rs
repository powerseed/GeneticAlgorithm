use rand::{Rng, RngCore};

pub struct Bird {
    pub(crate) position: nalgebra::Point2<f32>,
    pub(crate) rotation: nalgebra::Rotation2<f32>,
    pub(crate) speed: f32,
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