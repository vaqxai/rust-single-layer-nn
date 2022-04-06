use crate::math;

pub struct Perceptron {
	pub inputs: Vec<f64>,
	pub weights: Vec<f64>,
	pub bias: f64,
	pub learning_rate: f64,
}

impl Perceptron {

	pub fn new(dimensions: usize, learning_rate: f64) -> Perceptron {
		let mut weights = Vec::new();
		let mut inputs = Vec::new();
		for _ in 0..(dimensions) {
			inputs.push(0.0);
			weights.push(math::random_f64());
		}
		Perceptron {
			inputs: inputs,
			weights: weights,
			bias: math::random_f64(),
			learning_rate: learning_rate,
		}
	}

	pub fn predict(&self, inputs: &Vec<f64>) -> f64 {
		let mut sum = 0.0;
		for i in 0..inputs.len() {
			sum += inputs[i] * self.weights[i];
		}

		// sum += self.bias; // Why?
		// math::sigmoid(sum)

		if sum < self.bias {
			0.0
		} else {
			1.0
		}

	}

	pub fn train(&mut self, inputs: Vec<f64>, expected_output: f64) {
		let output = self.predict(&inputs);
		let error = expected_output - output;
		for i in 0..self.weights.len() {
			self.weights[i] += self.learning_rate * error * inputs[i];
		}
		self.bias -= self.learning_rate * error;
		println!("Training step: weights: {:?}, bias: {}", self.weights, self.bias);
	}
}