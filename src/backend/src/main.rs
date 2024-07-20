mod magicsquare;
mod magicsquare_solver;

use crate::magicsquare::*;
use crate::magicsquare_solver::*;

fn main() {
    let mut ms3x3 = MagicSquare::new(3);
    if MagicSquareSolver::solve(&mut ms3x3) {
        println!("Solution Found: \n{}", ms3x3);
    } else {
        println!("No solution found.");
    }
}
