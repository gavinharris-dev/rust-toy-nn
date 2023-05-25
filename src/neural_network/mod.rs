// pub use crate::matrix::Matrix;

pub mod NeuralNetwork {
    use serde::{Serialize, Deserialize};
    pub use crate::matrix::Matrix::Matrix;
    // This is the actual Neural Network


    #[derive(Serialize, Deserialize, Debug)]
    pub struct NeuralNetwork {
        input_nodes: usize,
        output_nodes: usize,

        learning_rate: f64,

        weights_ih: Matrix,
        weights_ho: Matrix,

        bias_h: Matrix,
        bias_o: Matrix,
    }

    impl NeuralNetwork {
        pub fn sigmoid(x: f64, _i: usize, _j: usize) -> f64 {
            1.0 / (1.0 + (-x).exp())
        }
        pub fn d_sigmoid(x: f64, _i: usize, _j: usize) -> f64 {
            x * (1.0 - x)
        }

        pub fn new(
            input_node_count: usize,
            hidden_node_count: usize,
            output_node_count: usize,
        ) -> NeuralNetwork {
            let weights_ih = Matrix::random(hidden_node_count, input_node_count);
            let weights_ho = Matrix::random( output_node_count, hidden_node_count);

            let bias_h = Matrix::random(hidden_node_count, 1);
            let bias_o = Matrix::random(output_node_count, 1);

            NeuralNetwork {
                input_nodes: input_node_count,
                output_nodes: output_node_count,
                weights_ih,
                weights_ho,
                bias_h,
                bias_o,
                learning_rate: 0.05,
            }
        }

        pub fn serialize(&self) -> String {
            serde_json::to_string(&self).unwrap()
        }

        pub fn feed_forward(&self, inputs: Vec<f64>) -> Vec<Vec<f64>> { 
            assert!(inputs.len() == self.input_nodes, "Input length does not match input nodes P={} N={}", inputs.len(), self.input_nodes);
            
            let input_nodes = Matrix::new(inputs, self.input_nodes, 1);
            println!("Input Nodes: {:?}", input_nodes);

            let mut hidden_nodes = self.weights_ih.multiply(&input_nodes);
            hidden_nodes.mut_add(&self.bias_h);
            hidden_nodes.mut_map(NeuralNetwork::sigmoid);
            println!("Hidden Nodes: {:?}", hidden_nodes);

            let mut output = self.weights_ho.multiply(&hidden_nodes);
            output.mut_add(&self.bias_o);
            output.mut_map(NeuralNetwork::sigmoid);

            println!("Output Nodes: {:?}", output);

            output.get_result()
        }

        pub fn train(&mut self, input: Vec<f64>, target: Vec<f64>) -> () {

            assert!(input.len() == self.input_nodes);
            let inputs = Matrix::new(input, self.input_nodes, 1);
            let targets = Matrix::new(target, self.output_nodes, 1);

            // println!("Weights IH: {:?}", self.weights_ih);
            // println!("Inputs: {:?}", inputs);
            // println!("bias_h: {:?}", self.bias_h);

            let mut hidden_nodes = self.weights_ih.multiply(&inputs);
            // println!("hidden_nodes: {:?}", hidden_nodes);
            hidden_nodes.mut_add(&self.bias_h);
            // println!("hidden_nodes: {:?}", hidden_nodes);
            hidden_nodes.mut_map(NeuralNetwork::sigmoid);
            // println!("hidden_nodes: {:?}", hidden_nodes);

            let mut output = self.weights_ho.multiply(&hidden_nodes);
            // println!("output: {:?}", output);
            output.mut_add(&self.bias_o);
            // println!("output: {:?}", output);
            output.mut_map(NeuralNetwork::sigmoid);
            // println!("output: {:?}", output);


            let mut output_errors = targets.subtract(&output);
            // println!("output_errors: {:?}", output_errors);
            let mut gradients = output.map(NeuralNetwork::d_sigmoid);
            // println!("gradients: {:?}", gradients);
            gradients.mut_multiply(&output_errors);
            // println!("gradients: {:?}", gradients);
            gradients.mut_multiply_scalar(self.learning_rate);
            // println!("gradients: {:?}", gradients);

            // println!("bias_o: {:?}", self.bias_o);
            self.bias_o.mut_add(&gradients);

            gradients.mut_multiply(&hidden_nodes.transpose());
            // println!("gradients: {:?}", gradients);

            // println!("weights_ho: {:?}", self.weights_ho);
            self.weights_ho.mut_add(&gradients);
            // println!("weights_ho: {:?}", self.weights_ho);


            let hidden_errors = self.weights_ho.transpose().multiply(&output_errors);
            let mut hidden_gradients = hidden_nodes.map(NeuralNetwork::d_sigmoid);
            // println!("hidden_gradients: {:?}", hidden_gradients);
            // println!("hidden_errors: {:?}", hidden_errors);
            hidden_gradients.mut_multiply_simple(&hidden_errors); // ERROR HERE
            // println!("hidden_gradients: {:?}", hidden_gradients);
            hidden_gradients.mut_multiply_scalar(self.learning_rate);
            // println!("hidden_gradients: {:?}", hidden_gradients);
            // println!("bias_h: {:?}", self.bias_h);
            self.bias_h.mut_add(&hidden_gradients);

            hidden_gradients.mut_multiply(&inputs.transpose());
            self.weights_ih.mut_add(&hidden_gradients);

        }

        pub fn trainBatch(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>) -> () {

            assert!(inputs.len() == targets.len());

            for i in 0..inputs.len() {
                self.train(inputs[i].clone(), targets[i].clone());
            }

        }



    }
}
