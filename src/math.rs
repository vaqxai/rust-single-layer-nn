use rand::prelude::*;

pub fn random_f64() -> f64 {
	let mut rng = rand::thread_rng();
	rng.gen::<f64>()
}

pub fn _sigmoid(x: f64) -> f64 {
	1.0 / (1.0 + (-x).exp())
}