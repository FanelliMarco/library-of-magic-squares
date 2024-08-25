use magic_squares::prelude::*;

fn main() {
    let mut ms = MagicSquare::new(10);
    let solver = SimulatedAnnealingSolver::new(1_000_000, 100.0, 0.995);
    let solver = MagicSquareSolver::new(Box::new(solver));
    loop {
        if solver.solve(&mut ms) {
            println!("Solution found:");
            println!("{}", ms);
            println!("Is valid magic square: {}", is_magic_square(&ms));
            break;
        } else {
            println!("Number of violations: {}", ms.get_num_of_violated());
        }
    }
}
