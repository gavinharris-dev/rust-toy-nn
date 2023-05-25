

mod matrix;
mod neural_network;
use rand::Rng;

fn main() {
    let training_data = vec![
        (vec![0.0, 0.0], vec![0.0]),
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 1.0], vec![0.0]),
        (vec![1.0, 0.0], vec![1.0]),
    ];


    let mut network: neural_network::NeuralNetwork::NeuralNetwork = neural_network::NeuralNetwork::NeuralNetwork::new(2, 12, 1);

    
    println!("{}", network.serialize());

    let mut rng = rand::thread_rng();
    for _i in 0..300000 {
        let index = rng.gen_range(0..training_data.len());

        let data = training_data[index].clone();

        network.train(data.0, data.1);
    }

    println!("{}", network.serialize());
    let guess = network.feed_forward(vec![1.0, 0.0]);
    println!("1 xor 0 === 1: {:?}", guess);

    let guess = network.feed_forward(vec![0.0, 0.0]);
    println!("0 xor 0 === 0: {:?}", guess);
    let guess = network.feed_forward(vec![0.0, 1.0]);
    println!("0 xor 1 === 1: {:?}", guess);
    let guess = network.feed_forward(vec![1.0, 1.0]);
    println!("1 xor 1 === 0: {:?}", guess);

}
