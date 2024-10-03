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

    // can also write the same with the built in folds
    // pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
    //     self.layers
    //         .iter()
    //         .fold(inputs, |inputs, layer| layer.propagate(inputs))
    // }
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>
}

// numbers will have to be pushed through each layer
impl Layer {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut outputs = outputs;
        for neuro in &self.neurons {
            let output = neuron.propagate(&inputs);
            outputs.push(output);
        }
        outputs
    }
}

#[derive(Debug)]
struct Neuron {
    bias:f3,
    weights: Vec<f32>,
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        // macro panics if two are not equal - terminate with error
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }
}