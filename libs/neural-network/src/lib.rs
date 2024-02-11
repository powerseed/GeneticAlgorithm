mod layer;
mod neuron;

use std::iter::once;
use crate::layer::Layer;

pub struct Network {
    layers: Vec<Layer>
}
impl Network {
    pub fn random(eye_cell_count: usize) -> Self {
        let neurons_per_layer = vec![eye_cell_count, 2 * eye_cell_count, 2];

        let layers = neurons_per_layer
            .windows(2)
            .map(|counts| Layer::random(counts[0], counts[1]))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers.iter().fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn get_weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }

    pub fn from_weights(neuron_count_in_each_layer: &[usize], weights: impl IntoIterator<Item = f32>) -> Self {
        let mut weights = weights.into_iter();

        let layers = neuron_count_in_each_layer
            .windows(2)
            .map(|neuron_counts| Layer::from_weights(neuron_counts[0], neuron_counts[1], &mut weights))
            .collect();

        Self {
            layers
        }
    }

    pub fn from_chromosome(chromosomes: Vec<f32>, eye_cell_count: usize) -> Self {
        Self::from_weights(
            &vec![eye_cell_count, eye_cell_count * 2, 2],
            chromosomes,
        )
    }
}