pub mod backtrack;
pub mod gaussian_elimination;

use crate::square::magic::MagicSquare;

pub trait SolvingAlgorithm {
    fn solve(&self, magic_square: &mut MagicSquare) -> bool;
}

pub struct MagicSquareSolver {
    algorithm: Box<dyn SolvingAlgorithm>,
}

impl MagicSquareSolver {
    pub fn new(algorithm: Box<dyn SolvingAlgorithm>) -> Self {
        MagicSquareSolver { algorithm }
    }

    pub fn solve(&self, magic_square: &mut MagicSquare) -> bool {
        self.algorithm.solve(magic_square)
    }
}
