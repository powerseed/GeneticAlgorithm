use rand::{Rng, RngCore};
use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
#[derive(Serialize)]
pub struct World {
    birds: Vec<Bird>,
    foods: Vec<Food>
}
impl World {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let birds = (0..10).map(|_| Bird::new(&mut rng)).collect();
        let foods = (0..20).map(|_| Food::new(&mut rng)).collect();

        Self {
            birds,
            foods
        }
    }
}
#[derive(Serialize)]
pub struct Bird {
    position: nalgebra::Point2<f32>,
    rotation: f32,
    speed: f32
}
impl Bird {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen::<nalgebra::Rotation2<f32>>().angle(),
            speed: rng.gen()
        }
    }
}
#[derive(Serialize)]
pub struct Food {
    position: nalgebra::Point2<f32>
}
impl Food {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen()
        }
    }
}
#[wasm_bindgen]
pub fn create_new_world() -> Result<JsValue, JsValue> {
    Ok(serde_wasm_bindgen::to_value(&World::new())?)
}