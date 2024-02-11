use rand::{Rng, RngCore};

pub struct Food {
    pub(crate) position: nalgebra::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self { position: rng.gen() }
    }
    pub fn get_position(&self) -> nalgebra::Point2<f32> {
        self.position
    }
}