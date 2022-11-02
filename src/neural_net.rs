use ndarray::{Array, Array2, Dimension};
use ndarray_rand::rand_distr::StandardNormal;
use ndarray_rand::RandomExt;
use std::iter::zip;

#[derive(Debug)]
pub struct Model {
    pub w: Vec<Array2<f64>>, // weights
    pub b: Vec<Array2<f64>>, // biases
}

impl Model {
    pub fn init(isize: usize, osize: usize, hidden_layers: Vec<usize>) -> Model {
        let mut m = Model {
            w: Vec::<Array2<f64>>::new(),
            b: Vec::<Array2<f64>>::new(),
        };

        m.w.push(Array2::random((hidden_layers[0], isize), StandardNormal));
        m.b.push(Array2::zeros((hidden_layers[0], 1)));
        for x in 0..hidden_layers.len() - 1 {
            m.w.push(Array2::random(
                (hidden_layers[x + 1], hidden_layers[x]),
                StandardNormal,
            ));
            m.b.push(Array2::zeros((hidden_layers[x + 1], 1)));
        }
        m.w.push(Array2::random(
            (osize, hidden_layers[hidden_layers.len() - 1]),
            StandardNormal,
        ));
        m.b.push(Array2::zeros((osize, 1)));

        return m;
    }

    pub fn train_mini_batch(
        &mut self,
        input_batch: &Vec<Array2<f64>>,
        output_batch: &Vec<Array2<f64>>,
        lr: f64,
    ) {
        let mut w_grads_batch_aggregate: Vec<Array2<f64>> = (0..self.w.len())
            .map(|l| Array2::<f64>::zeros(self.w[l].dim()))
            .collect();
        let mut b_grads_batch_aggregate: Vec<Array2<f64>> = (0..self.b.len())
            .map(|l| Array2::<f64>::zeros(self.b[l].dim()))
            .collect();

        for (x, y) in zip(input_batch, output_batch) {
            let (w_grad, b_grad, _) = self.backprop(x, y);

            for i in 0..w_grad.len() {
                w_grads_batch_aggregate[i] = &w_grads_batch_aggregate[i] + &w_grad[i];
            }

            for i in 0..b_grad.len() {
                b_grads_batch_aggregate[i] = &b_grads_batch_aggregate[i] + &b_grad[i];
            }
        }

        for i in 0..self.w.len() {
            self.w[i] =
                &self.w[i] - (lr * (&w_grads_batch_aggregate[i] / input_batch.len() as f64));
            self.b[i] =
                &self.b[i] - (lr * (&b_grads_batch_aggregate[i] / input_batch.len() as f64));
        }
    }

    pub fn feed_forward(&mut self, input: &Array2<f64>) -> Array2<f64> {
        let mut a = Array2::<f64>::zeros((1, 1));
        let mut next_in = input;
        for (w, b) in zip(&self.w, &self.b) {
            let z = w.dot(next_in) + b;
            a = sigmoid_activation(&z);
            next_in = &a;
        }
        return a;
    }

    pub fn backprop(
        &mut self,
        x: &Array2<f64>,
        y: &Array2<f64>,
    ) -> (Vec<Array2<f64>>, Vec<Array2<f64>>, f64) {
        let mut w_grad: Vec<Array2<f64>> = (0..self.w.len())
            .map(|l| Array2::<f64>::zeros(self.w[l].dim()))
            .collect();
        let mut b_grad: Vec<Array2<f64>> = (0..self.b.len())
            .map(|l| Array2::<f64>::zeros(self.b[l].dim()))
            .collect();

        // Feed forward
        let mut outputs = Vec::<Array2<f64>>::new();
        let mut a = Vec::<Array2<f64>>::new();
        let mut next_in = x;
        for (w, b) in zip(&self.w, &self.b) {
            let z = w.dot(next_in) + b;
            outputs.push(sigmoid_activation(&z));
            next_in = &outputs[outputs.len() - 1];
            a.push(z);
        }

        let layers_len = outputs.len();

        // Calculate all err_rates
        let output_diff = &outputs[layers_len - 1] - y;
        let mut delta = (&output_diff) * &sigmoid_derivative(&outputs[layers_len - 1]);
        for l in (0..layers_len - 1).rev() {
            let sd = sigmoid_derivative(&outputs[l]);
            let _delta = &self.w[l + 1].t().dot(&delta) * &sd;
            if l == 0 {
                w_grad[l] = _delta.dot(&x.t());
            } else {
                w_grad[l] = _delta.dot(&outputs[l - 1].t());
            }
            b_grad[l] = _delta.clone();
            delta = _delta;
        }

        let cost = (&output_diff * &output_diff / 2.).mean().unwrap();

        return (w_grad, b_grad, cost);
    }
}

fn sigmoid_activation<T: Dimension>(val: &Array<f64, T>) -> Array<f64, T> {
    val.mapv(|val| 1.0 / (1.0 + f64::exp(-val)))
}

fn sigmoid_derivative<T: Dimension>(val: &Array<f64, T>) -> Array<f64, T> {
    val.mapv(|sigmoid_val| sigmoid_val * (1.0 - sigmoid_val))
}