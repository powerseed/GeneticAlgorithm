use crate::neuron;

pub struct Layer {
    pub neurons: Vec<neuron::Neuron>
}
impl Layer {
    pub fn random(input_count: usize, neuron_count: usize) -> Self {
        let neurons = (0..neuron_count)
            .map(|_| neuron::Neuron::random(input_count))
            .collect();

        Self { neurons }
    }

    pub fn from_weights(input_count: usize, neuron_count: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let neurons = (0..neuron_count)
            .map(|_| neuron::Neuron::from_weights(input_count, weights))
            .collect();

        Self { neurons }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}