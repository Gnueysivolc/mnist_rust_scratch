use crate::model::NeuralNet;
use crate::utils::display_image;
use ndarray::Array1;
use std::io;

pub fn user_testing_single(
    test_xs: &Vec<Array1<f32>>,
    test_ys: &Vec<Array1<f32>>,
    net: &NeuralNet,
) {
    println!("\n === single sample prediction demo ===");

    println!("pls input a number between 1 - 10000, for testing the model");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let user_input: i32 = input.trim().parse().expect("please enter a number");

    if user_input > 10000 || user_input == 0 || user_input < 0 {
        println!("pls input a valid number, the range of testing element is from 1 - 10000");
        return;
    }

    let n = (user_input - 1) as usize;

    let x_array = &test_xs[n];
    let y_array = &test_ys[n];

    let true_class = y_array.iter().position(|&v| v > 0.5).unwrap();

    display_image(x_array);

    let (predicted, probabilities) = net.predict_single(x_array);

    println!("true label of {}th test: {}", n, true_class);
    println!("predicted of {}th test: {}", n, predicted);
    println!("confidence per class:");
    probabilities.iter().enumerate().for_each(|(i, &prob)| {
        println!(" {}: {:>5.2}%", i, prob * 100.0);
    });

    println!(
        "\n {}",
        if predicted == true_class {
            "correct"
        } else {
            "wrong"
        }
    );
}
