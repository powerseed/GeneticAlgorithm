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
            for (index, neuron_in_prev_layer) in two_layers[0].neurons.iter().enumerate() {
                two_layers[1].neurons
                    .iter()
                    .for_each(|mut neuron_in_next_layer| {
                        neuron_in_next_layer.receive(neuron_in_prev_layer.value, index)
                    })
            }
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
    pub fn receive(&mut self, prev_value: f32, prev_index: usize) {
        let output = prev_value * self.weights[prev_index];
        self.value = (output + self.bias).max(0.0);
    }
}