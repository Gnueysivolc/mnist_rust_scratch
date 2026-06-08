use ndarray::Array1;

pub fn one_hot_encode(label: usize, num_class: usize) -> Array1<f32> {
    let mut y = Array1::zeros(num_class);
    y[label] = 1.0;
    y
}

pub fn accuracy(predictions: &[usize], labels: &[usize]) -> f32 {
    let correct = predictions
        .iter()
        .zip(labels.iter())
        .filter(|&(&pred, &label)| pred == label)
        .count();
    correct as f32 / labels.len() as f32
}

pub fn display_image(x: &Array1<f32>) {
    println!("\n input image (28 x 28):");

    for i in 0..28 {
        for j in 0..28 {
            let pixel = x[i * 28 + j];
            let c = if pixel > 0.8 {
                "██"
            } else if pixel > 0.6 {
                "▓▓"
            } else if pixel > 0.4 {
                "▒▒"
            } else if pixel > 0.2 {
                "░░"
            } else {
                "  "
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}
