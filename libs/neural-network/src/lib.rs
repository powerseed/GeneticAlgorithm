use rand::prelude::*;
pub struct Network {
    layers: Vec<Layer>
}
impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self {
            layers
        }
    }
    pub fn propagate(&self) -> ! {
        self.layers.windows(2).map(|two_layers| {
            two_layers[0].propagate(&two_layers[1]);
        })
    }
}
pub struct Layer {
    neurons: Vec<Neuron>
}
impl Layer {
    pub fn new(neurons: Vec<Neuron>) -> Self {
        Self {
            neurons
        }
    }
    pub fn propagate(&self, next_layer: &Layer) {
        for (index, neuron) in self.neurons.iter().enumerate() {
            next_layer.neurons
                .iter()
                .for_each(|mut neuron_in_next_layer| {
                    neuron.propagate(index, &mut neuron_in_next_layer);
                })
        }
    }
}

pub struct Neuron {
    bias: f32,
    weights: Vec<f32>,
    value: f32
}
impl Neuron {
    pub fn new(weight_len: &u32) -> Self {
        let bias = thread_rng().gen_range(-1.0..=1.0);
        let weights = (0..weight_len).map(|_| thread_rng().gen_range(-1.0..=1.0)).collect();

        Self {
            bias,
            weights,
            value: 0.0
        }
    }
    pub fn propagate(&self, index: usize, next_neuron: &mut Neuron) {
        let output = self.value * next_neuron.weights[index];
        next_neuron.value = (output + next_neuron.bias).max(0.0);
    }
}