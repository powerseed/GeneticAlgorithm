use rand::{Rng, thread_rng};

pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<f32>
}
impl Neuron {
    pub fn random(input_count: usize) -> Self {
        let mut rng = thread_rng();
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..input_count).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        Self {
            bias,
            weights
        }
    }
    pub fn from_weights(input_count: usize, weights: &mut dyn Iterator<Item = f32>) -> Self
    {
        Self {
            bias: weights.next().expect("Not enough weights. "),
            weights: (0..input_count).map(|_| {
                weights.next().expect("Not enough weights. ")
            }).collect()
        }
    }
    pub fn propagate(&self, inputs: &Vec<f32> ) -> f32 {
        if inputs.len() != self.weights.len() {
            panic!();
        }

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| {
                input * weight
            })
            .sum::<f32>();

        (output + self.bias).max(0.0)
    }
}