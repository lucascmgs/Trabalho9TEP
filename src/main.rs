extern crate rand;
use rand::Rng;
use std::f64;

struct Sorteador {
    generator: rand::rngs::ThreadRng,
    weights: Vec<usize>,
    probabilities: Vec<f64>,
}

impl Sorteador {
    fn new(given_weights: Vec<usize>) -> Sorteador {
        let mut sorteador = Sorteador {
            generator: rand::thread_rng(),
            weights: given_weights,
            probabilities: Vec::new(),
        };
        sorteador.construct_probabilities();
        sorteador
    }

    pub(self) fn construct_probabilities(&mut self) {
        assert!(self.weights.len() > 1);
        assert!(self.probabilities.len() == 0);
        let mut sum = 0;
        for i in &self.weights {
            sum = sum + *i;
        }
        let sum = sum as f64;
        let mut probabilities = Vec::new();
        probabilities.push((self.weights[0] as f64)/(sum as f64));
        for i in 1..self.weights.len() {
            probabilities.push(probabilities[i - 1] + (self.weights[i] as f64) / sum);
        }
        self.probabilities = probabilities;
        println!("Probabilities: {:?}", self.probabilities);
    }

    fn choose_value(&self, value: f64) -> usize {
        if value <= self.probabilities[0] {
            return 0;
        }

        let mut start_index = 0;
        let mut end_index = self.probabilities.len();
        loop {
            let index = (start_index + end_index) / 2;
            let vector_value = self.probabilities[index];
            if value > vector_value {
                start_index = index;
            } else {
                end_index = index;
            }
            if (end_index - start_index) <= 1 {
                break;
            }
        }
        end_index
    }

    fn sample(&mut self) -> usize {
        let value = self
            .generator
            .gen_range(0.0, self.probabilities.last().unwrap());
        self.choose_value(value)
    }
}

fn compute_trios(sorteador: &mut Sorteador, n_iterations: usize) -> f64 {
    let mut count: usize = 0;
    for i in 0..n_iterations {
        let results = (sorteador.sample(), sorteador.sample(), sorteador.sample());
        if results.0 == results.1 && results.1 == results.2 {
            count = count + 1;
        }
    }
    let count = count as f64;
    let total = n_iterations as f64;
    count / total
}

fn compute_two_pairs(sorteador: &mut Sorteador, n_iterations: usize) -> f64 {
    let mut count: usize = 0;

    for i in 0..n_iterations {
        let first_pair = (sorteador.sample(), sorteador.sample());
        let second_pair = (sorteador.sample(), sorteador.sample());
        if first_pair.0 == first_pair.1 && second_pair.0 == second_pair.1 {
            count = count + 1;
        }
    }
    let count = count as f64;
    let total = n_iterations as f64;
    count / total
}

enum CheckMethod {
    Trio,
    TwoPairs,
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn simulate_with_weights(sorteador: &mut Sorteador, method: CheckMethod) -> f64 {
        let n_iterations = 1000000;
        match method {
            CheckMethod::Trio => compute_trios(sorteador, n_iterations),
            _ => compute_two_pairs(sorteador, n_iterations),
        }
    }

    fn simulate_multiple_draws(sorteador: &mut Sorteador, n_iterations: usize) -> Vec<f64> {
        let mut total = 0;
        let mut result: Vec<f64> = Vec::new();
        result.resize(sorteador.weights.len(), 0.0);
        for _ in 0..n_iterations{
            total = total + 1;
            let sample = sorteador.sample();
            result[sample] = result[sample] + 1.0;
        }

        for i in 0..sorteador.weights.len(){
            result[i] = result[i]/(total as f64);
        }

        result
    }

    #[test]
    fn test_honest_coin() {
        let weights = vec![1, 1];
        let mut sorteador = Sorteador::new(weights);
        let trio_value = simulate_with_weights(&mut sorteador, CheckMethod::Trio);
        let two_pairs_value = simulate_with_weights(&mut sorteador, CheckMethod::TwoPairs);
        println!("Moeda: trio: {} pares: {}", trio_value, two_pairs_value);
        assert!((two_pairs_value - trio_value).abs() < 0.05);
    }
    #[test]
    fn test_honest_die() {
        let weights = vec![1, 1, 1, 1, 1, 1];
        let mut sorteador = Sorteador::new(weights);
        let trio_value = simulate_with_weights(&mut sorteador, CheckMethod::Trio);
        let two_pairs_value = simulate_with_weights(&mut sorteador, CheckMethod::TwoPairs);
        println!("Dado: trio: {} pares: {}", trio_value, two_pairs_value);
        assert!((two_pairs_value - trio_value).abs() < 0.05);
    }

    #[test]
    fn test_consistency() {
        let weights = vec![1, 1, 1, 1, 1];
        let mut sorteador = Sorteador::new(weights);

        let n_iterations = 100000;
        let result = simulate_multiple_draws(&mut sorteador, n_iterations);
        println!("{:?}", result);
        for i in 0..result.len() {
            assert!((result[i] - 0.2).abs() < 0.05);
        }
    }
}
