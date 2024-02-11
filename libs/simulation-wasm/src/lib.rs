use rand::thread_rng;
use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use lib_simulation;

#[wasm_bindgen]
pub struct Simulation {
    sim: lib_simulation::Simulation
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = lib_simulation::Simulation::random(&mut rng);

        Self { sim }
    }

    pub fn get_world(&self) -> JsValue {
        let world = World::from(self.sim.get_world());
        JsValue::from_serde(&world).unwrap()
    }

    pub fn step(&mut self) {
        let mut rng = thread_rng();
        self.sim.step(&mut rng);
    }
}
#[derive(Serialize)]
pub struct World {
    pub birds: Vec<Bird>,
    pub foods: Vec<Food>
}
impl From<&lib_simulation::world::World> for World {
    fn from(source: &lib_simulation::world::World) -> Self {
        Self {
            birds: source.get_birds().iter().map(|bird| Bird::from(bird)).collect(),
            foods: source.get_foods().iter().map(|food| Food::from(food)).collect()
        }
    }
}
#[derive(Serialize)]
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub rotation: f32
}
impl From<&lib_simulation::bird::Bird> for Bird {
    fn from(source: &lib_simulation::bird::Bird) -> Self {
        Self {
            x: source.get_position().x,
            y: source.get_position().y,
            rotation: source.get_rotation().angle()
        }
    }
}
#[derive(Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32
}

impl From<&lib_simulation::food::Food> for Food {
    fn from(source: &lib_simulation::food::Food) -> Self {
        Self {
            x: source.get_position().x,
            y: source.get_position().y
        }
    }
}