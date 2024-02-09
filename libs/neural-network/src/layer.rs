use crate::neuron;

pub struct Layer {
    neurons: Vec<neuron::Neuron>
}
impl Layer {
    pub fn random(input_count: i32, neuron_count: i32) -> Self {
        let neurons = (0..neuron_count).map(|_| neuron::Neuron::random(input_count)).collect();
        Self {
            neurons
        }
    }
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons.iter().map(|neuron| {
            neuron.propagate(&inputs)
        }).collect()
    }
}