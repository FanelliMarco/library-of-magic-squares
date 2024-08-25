pub mod solver;
pub mod square;
pub mod utils;

pub mod prelude;

// Re-export key items for easier use
pub use solver::backtrack::BacktrackSolver;
pub use solver::gaussian_elimination::GaussianEliminationSolver;
pub use solver::simulated_annealing::SimulatedAnnealingSolver;
pub use solver::{MagicSquareSolver, SolvingAlgorithm};
pub use square::magic::MagicSquare;
