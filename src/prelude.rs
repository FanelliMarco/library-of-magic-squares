pub use crate::solver::backtrack::BacktrackSolver;
pub use crate::solver::gaussian_elimination::GaussianEliminationSolver;
pub use crate::solver::simulated_annealing::SimulatedAnnealingSolver;
pub use crate::solver::{MagicSquareSolver, SolvingAlgorithm};
pub use crate::square::magic::MagicSquare;
pub use crate::utils::validation::is_magic_square;

pub mod prelude {
    pub use super::*;
}
