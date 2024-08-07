use crate::square::magic::MagicSquare;
use crate::utils::validation::is_magic_square;

pub struct MagicSquareSolver;

impl MagicSquareSolver {
    pub fn new() -> Self {
        MagicSquareSolver
    }

    pub fn solve(&self, magic_square: &mut MagicSquare) -> bool {
        let n = magic_square.order();
        let mut used = vec![false; (n * n + 1) as usize];

        self.backtrack(magic_square, 0, 0, &mut used)
    }

    fn backtrack(
        &self,
        square: &mut MagicSquare,
        row: usize,
        col: usize,
        used: &mut Vec<bool>,
    ) -> bool {
        let n = square.order();

        if row == n {
            return is_magic_square(square);
        }

        let next_row = if col == n - 1 { row + 1 } else { row };
        let next_col = (col + 1) % n;

        for num in 1..=(n * n) as i8 {
            if !used[num as usize] {
                if square.set(row, col, num).is_ok() {
                    used[num as usize] = true;

                    if self.backtrack(square, next_row, next_col, used) {
                        return true;
                    }

                    used[num as usize] = false;
                    let _ = square.set(row, col, 0);
                }
            }
        }

        false
    }
}
