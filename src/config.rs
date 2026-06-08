// src/config.rs

pub const EPOCHS: usize = 5;
pub const LEARNING_RATE: f32 = 0.01;
pub const CHUNK_SIZES: usize = 10_000;
pub const HIDDEN_SIZE: usize = 40;
pub const DEMO_INDEX: usize = 42;

pub const TRAIN_PATH: &str = "src/data/mnist_train.csv";
pub const TEST_PATH: &str = "src/data/mnist_test.csv";
