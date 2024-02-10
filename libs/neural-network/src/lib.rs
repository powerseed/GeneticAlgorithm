mod layer;
mod neuron;

use rand::prelude::*;

pub struct Network {
    layers: Vec<layer::Layer>
}
impl Network {
    pub fn random(neurons_per_layer: Vec<usize>) -> Self {
        let layers = neurons_per_layer.windows(2).map(|counts| {
            layer::Layer::random(counts[0], counts[1])
        }).collect();

        Self {
            layers
        }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers.iter().fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}