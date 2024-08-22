use magic_squares::prelude::*;

fn main() {
    loop {
        let result = simulated_annealing(5);
        if is_magic_square(&result) {
            println!("Best magic square found:");
            println!("{}", result);
            println!("Violations: {}", result.get_num_of_violated());
            println!("Check if valid magic square: {}", is_magic_square(&result));
            break;
        } else {
            simulated_annealing(5);
        }
    }
}
