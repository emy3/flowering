// we will be propagating numbers here

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, input: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>
}

// numbers will have to be pushed through each layer
impl Layer {
    pub fn propagate(&self, input: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}

#[derive(Debug)]
struct Neuron {
    bias:f3,
    weights: Vec<f32>,
}