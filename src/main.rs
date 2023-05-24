

mod matrix;
mod neural_network;
use rand::Rng;

fn main() {
    let training_data = vec![
        (vec![0.0, 0.0], vec![0.0]),
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 0.0], vec![1.0]),
        (vec![1.0, 1.0], vec![0.0]),
    ];


    let mut network: neural_network::NeuralNetwork::NeuralNetwork = neural_network::NeuralNetwork::NeuralNetwork::new(2, 3, 1);

    // println!("{}", network.serialize());


    let mut rng = rand::thread_rng();
    for _i in 0..90000 {
        let index = rng.gen_range(0..training_data.len());

        let data = training_data[index].clone();

        network.train(data.0, data.1);
    }

    println!("{}", network.serialize());
    let guess = network.feed_forward(vec![1.0, 0.0]);
    println!("{:?}", guess);

    let guess = network.feed_forward(vec![0.0, 0.0]);
    println!("{:?}", guess);
    let guess = network.feed_forward(vec![0.0, 1.0]);
    println!("{:?}", guess);
    let guess = network.feed_forward(vec![1.0, 1.0]);
    println!("{:?}", guess);

}
