use crate::solver::SolvingAlgorithm;
use crate::square::magic::MagicSquare;
use rand::Rng;

pub struct SimulatedAnnealingSolver {
    max_iterations: usize,
    initial_temperature: f64,
    cooling_rate: f64,
}

impl SimulatedAnnealingSolver {
    pub fn new(max_iterations: usize, initial_temperature: f64, cooling_rate: f64) -> Self {
        SimulatedAnnealingSolver {
            max_iterations,
            initial_temperature,
            cooling_rate,
        }
    }
}

impl SolvingAlgorithm for SimulatedAnnealingSolver {
    fn solve(&self, magic_square: &mut MagicSquare) -> bool {
        let mut rng = rand::thread_rng();
        let mut current_state = MagicSquare::gen_rand_square(magic_square.order());
        let mut current_energy = current_state.get_num_of_violated() as f64;
        let mut best_state = current_state.clone();
        let mut best_energy = current_energy;
        let mut temperature = self.initial_temperature;

        for iteration in 0..self.max_iterations {
            let neighbor = current_state.get_successor();
            let neighbor_energy = neighbor.get_num_of_violated() as f64;

            let delta_energy = neighbor_energy - current_energy;

            if delta_energy < 0.0 || rng.gen::<f64>() < (-delta_energy / temperature).exp() {
                current_state = neighbor;
                current_energy = neighbor_energy;

                if current_energy < best_energy {
                    best_state = current_state.clone();
                    best_energy = current_energy;

                    if best_energy == 0.0 {
                        println!("Solution found at iteration {}", iteration);
                        break;
                    }
                }
            }

            temperature *= self.cooling_rate;
        }

        *magic_square = best_state;
        best_energy == 0.0
    }
}
