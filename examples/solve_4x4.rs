use magic_squares::prelude::*;

fn main() {
    let mut square = MagicSquare::new(4);
    let solver = MagicSquareSolver::new(Box::new(GaussianEliminationSolver::new()));

    if solver.solve(&mut square) {
        println!("Example of a valid Magic Square:");
        println!("{}", square);
        assert!(is_magic_square(&square));
    } else {
        println!("Failed to solve the Magic Square.");
    }
}
