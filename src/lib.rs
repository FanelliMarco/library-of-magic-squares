pub mod square;
pub mod solver;
pub mod utils;

pub mod prelude;

// Re-export key items for easier use
pub use square::magic::MagicSquare;
pub use solver::backtrack::MagicSquareSolver;
