use rand::prelude::SliceRandom;
use rand::Rng;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
pub struct MagicSquare {
    square: Vec<i8>,
    order: usize,
}

impl MagicSquare {
    pub fn new(order: usize) -> Self {
        MagicSquare {
            square: vec![0; order * order],
            order,
        }
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn get(&self, row: usize, col: usize) -> Option<i8> {
        if row < self.order && col < self.order {
            Some(self.square[row * self.order + col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: i8) -> Result<(), &'static str> {
        if row < self.order && col < self.order {
            self.square[row * self.order + col] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn to_2d_vec(&self) -> Vec<Vec<i8>> {
        self.square
            .chunks(self.order)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    pub fn magic_constant(&self) -> usize {
        self.order * (self.order * self.order + 1) / 2
    }
}

impl fmt::Display for MagicSquare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.to_2d_vec() {
            for &num in &row {
                write!(f, "{:4}", num)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl MagicSquare {
    pub fn gen_rand_square(order: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut square = MagicSquare::new(order);
        let mut values: Vec<i8> = (1..=(order * order) as i8).collect();
        values.shuffle(&mut rng);

        for i in 0..order {
            for j in 0..order {
                square.set(i, j, values[i * order + j]).unwrap();
            }
        }
        square
    }

    pub fn get_num_of_violated(&self) -> usize {
        let target_sum = self.magic_constant() as i32;
        let mut num_of_violated = 0;

        // Check rows
        for i in 0..self.order {
            let row_sum: i32 = (0..self.order)
                .map(|j| self.get(i, j).unwrap() as i32)
                .sum();
            if row_sum != target_sum {
                num_of_violated += 1;
            }
        }

        // Check columns
        for j in 0..self.order {
            let col_sum: i32 = (0..self.order)
                .map(|i| self.get(i, j).unwrap() as i32)
                .sum();
            if col_sum != target_sum {
                num_of_violated += 1;
            }
        }

        // Check main diagonal
        let main_diag_sum: i32 = (0..self.order)
            .map(|i| self.get(i, i).unwrap() as i32)
            .sum();
        if main_diag_sum != target_sum {
            num_of_violated += 1;
        }

        // Check non-main diagonal
        let non_main_diag_sum: i32 = (0..self.order)
            .map(|i| self.get(i, self.order - 1 - i).unwrap() as i32)
            .sum();
        if non_main_diag_sum != target_sum {
            num_of_violated += 1;
        }

        num_of_violated
    }

    pub fn get_successor(&self) -> Self {
        let mut rng = rand::thread_rng();
        let mut successor = self.clone();
        let (x1, y1) = (rng.gen_range(0..self.order), rng.gen_range(0..self.order));
        let (mut x2, mut y2) = (rng.gen_range(0..self.order), rng.gen_range(0..self.order));

        while x1 == x2 && y1 == y2 {
            x2 = rng.gen_range(0..self.order);
            y2 = rng.gen_range(0..self.order);
        }

        let temp = successor.get(x1, y1).unwrap();
        successor
            .set(x1, y1, successor.get(x2, y2).unwrap())
            .unwrap();
        successor.set(x2, y2, temp).unwrap();

        successor
    }
}

pub fn simulated_annealing(order: usize) -> MagicSquare {
    let mut rng = rand::thread_rng();
    let mut t = 100.0;
    let mut current_state = MagicSquare::gen_rand_square(order);
    let mut current_violations = current_state.get_num_of_violated();

    let mut best_state = current_state.clone();
    let mut best_violations = current_violations;

    let mut iteration = 0;

    while iteration < 1000 && best_violations > 0 && t > 0.0 {
        t *= 0.995; // Geometric cooling schedule

        // Generate a batch of 50 successors and select the best one
        let mut batch = Vec::new();
        for _ in 0..50 {
            let successor = current_state.get_successor();
            let violations = successor.get_num_of_violated();
            batch.push((successor, violations));
        }

        batch.sort_by(|a, b| a.1.cmp(&b.1));
        let (best_successor, best_successor_violations) = batch[0].clone();

        match best_successor_violations.cmp(&current_violations) {
            Ordering::Less => {
                current_state = best_successor;
                current_violations = best_successor_violations;
                if current_violations < best_violations {
                    best_state = current_state.clone();
                    best_violations = current_violations;
                }
            }
            _ => {
                let delta_e = (best_successor_violations - current_violations) as f64;
                let probability = (-delta_e / t).exp();
                if rng.gen::<f64>() < probability {
                    current_state = best_successor;
                    current_violations = best_successor_violations;
                }
            }
        }

        iteration += 1;
    }

    best_state
}
