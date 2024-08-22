use magic_squares::prelude::*;

fn main() {
    println!("Solving a 3x3 Magic Square");

    let mut ms3x3 = MagicSquare::new(3);
    let mut solver = MagicSquareSolver::new();

    if solver.solve(&mut ms3x3) {
        println!("Solution Found:");
        println!("{}", ms3x3);

        if is_magic_square(&ms3x3) {
            println!("Verified: This is a valid magic square!");
        } else {
            println!("Error: The solution is not a valid magic square.");
        }
    } else {
        println!("No solution found.");
    }
}
