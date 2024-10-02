// we will be propagating numbers here

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        // let mut inputs = inputs; //rebinding[shadowing]
        // mut inputs >> appears in so-called binding position 
        // bindings are local to the function
        for layer in &self::layers {
            inputs = layer.propagate(inputs);
        }
        inputs
    }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>
}

// numbers will have to be pushed through each layer
impl Layer {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}

#[derive(Debug)]
struct Neuron {
    bias:f3,
    weights: Vec<f32>,
}