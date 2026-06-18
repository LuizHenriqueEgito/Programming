use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::distr::{Distribution, Uniform};

const SEED: u64 = 42;

pub fn create_label_vector(size: usize) -> Vec<f32> {
    let mut rng = StdRng::seed_from_u64(SEED);
    let dist = Uniform::new_inclusive(0.0_f32, 1.0_f32).unwrap();
    (0..size).map(|_| dist.sample(&mut rng).round()).collect()
}

pub fn create_predict_label_vector(size: usize) -> Vec<f32> {
    let mut rng = StdRng::seed_from_u64(SEED);
    let dist = Uniform::new_inclusive(0.0_f32, 1.0_f32).unwrap();
    (0..size).map(|_| dist.sample(&mut rng)).collect()
}

pub fn create_target_vector(size: usize) -> Vec<f64> {
    let mut rng = StdRng::seed_from_u64(SEED);
    let dist = Uniform::new_inclusive(100.0_f64, 1000.0_f64).unwrap();
    (0..size).map(|_| dist.sample(&mut rng)).collect()
}

pub fn create_predict_target_vector(base: &[f64]) -> Vec<f64> {
    let mut rng = StdRng::seed_from_u64(SEED);
    let dist = Uniform::new_inclusive(-0.5_f64, 0.5_f64).unwrap();
    base.iter()
        .map(|&value| {
            let ruido = dist.sample(&mut rng);
            value * (1.0 + ruido)
        })
        .collect()
}

pub fn threshold_fn(
    y_hat: &Vec<f32>,
    threshold: f32
) -> Vec<f32> {
    y_hat
        .iter()
        .map(|&value| if value >= threshold { 1.0 } else { 0.0 })
        .collect()
}