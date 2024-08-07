//! Prelude for the magic_squares library.
//!
//! This module re-exports the most commonly used types and functions
//! from the library, allowing users to import them all at once.

pub use crate::square::magic::MagicSquare;
pub use crate::solver::backtrack::MagicSquareSolver;
pub use crate::utils::validation::is_magic_square;

// If you have any error types or result types, you might want to include them here as well.
// For example:
// pub use crate::error::MagicSquareError;

// If you have any commonly used traits, include them here:
// pub use crate::traits::SomeTrait;

/// A prelude-style import for convenient importing of common items.
///
/// # Example
///
/// ```
/// use magic_squares::prelude::*;
///
/// let mut square = MagicSquare::new(3);
/// let solver = MagicSquareSolver::new();
/// if solver.solve(&mut square) {
///     assert!(is_magic_square(&square));
/// }
/// ```
pub mod prelude {
    pub use super::*;
}
