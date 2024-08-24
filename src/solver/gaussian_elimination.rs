use super::SolvingAlgorithm;
use crate::square::magic::MagicSquare;
use crate::utils::validation::is_magic_square;
use std::collections::HashSet;

pub struct GaussianEliminationSolver;

impl SolvingAlgorithm for GaussianEliminationSolver {
    fn solve(&self, magic_square: &mut MagicSquare) -> bool {
        if magic_square.order() != 4 {
            return false; // This algorithm is specific to order 4 magic squares
        }

        let mut solutions = HashSet::new();
        let mut free_vars = vec![0; 7];
        self.find_all_solutions(&mut free_vars, 0, &mut solutions);

        println!("Found {} unique magic squares of order 4", solutions.len());

        if !solutions.is_empty() {
            *magic_square = solutions.iter().next().unwrap().clone();
            true
        } else {
            false
        }
    }
}

impl GaussianEliminationSolver {
    pub fn new() -> Self {
        GaussianEliminationSolver
    }

    fn find_all_solutions(
        &self,
        free_vars: &mut Vec<i8>,
        index: usize,
        solutions: &mut HashSet<MagicSquare>,
    ) {
        if index == 7 {
            if let Some(new_square) = self.construct_square(free_vars) {
                if is_magic_square(&new_square) {
                    solutions.insert(new_square);
                }
            }
            return;
        }

        for num in 1..=16 {
            if !free_vars.contains(&num) {
                free_vars[index] = num;
                self.find_all_solutions(free_vars, index + 1, solutions);
            }
        }
    }

    fn construct_square(&self, free_vars: &[i8]) -> Option<MagicSquare> {
        let [x1, x2, x3, x5, x6, x7, x9] = free_vars[..] else {
            return None;
        };

        let x4 = 34 - x1 - x2 - x3;
        let x8 = 34 - x5 - x6 - x7;
        let x10 = x1 + x5 + x9 - x4 - x7;
        let x11 = (2 * x4 + x7 + x8 - 2 * x1 - x5 - x6 - 2 * x9 + 34) / 2;
        let x12 = x1 + x6 + x11 - x4 - x8;
        let x13 = 34 - x1 - x5 - x9;
        let x14 = 34 - x2 - x6 - x10;
        let x15 = 34 - x3 - x7 - x11;
        let x16 = 34 - x4 - x8 - x12;

        let values = [
            x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16,
        ];

        if values.iter().all(|&x| x > 0 && x <= 16)
            && values.len() == values.iter().collect::<HashSet<_>>().len()
        {
            let mut new_square = MagicSquare::new(4);
            for (i, &val) in values.iter().enumerate() {
                new_square.set(i / 4, i % 4, val).unwrap();
            }
            Some(new_square)
        } else {
            None
        }
    }
}
