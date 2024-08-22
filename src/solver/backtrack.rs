use crate::square::magic::MagicSquare;
use crate::utils::validation::is_magic_square;

pub struct MagicSquareSolver {
    magic_sum: i32,
}

impl MagicSquareSolver {
    pub fn new() -> Self {
        MagicSquareSolver { magic_sum: 0 }
    }

    pub fn solve(&mut self, magic_square: &mut MagicSquare) -> bool {
        let n = magic_square.order();
        self.magic_sum = (n as i32) * ((n as i32) * (n as i32) + 1) / 2;
        let mut used = vec![false; (n * n + 1) as usize];

        self.backtrack(magic_square, 0, 0, &mut used, 0, 0)
    }

    fn backtrack(
        &self,
        square: &mut MagicSquare,
        row: usize,
        col: usize,
        used: &mut Vec<bool>,
        row_sum: i32,
        col_sum: i32,
    ) -> bool {
        let n = square.order();

        if row == n {
            return is_magic_square(square);
        }

        let next_row = if col == n - 1 { row + 1 } else { row };
        let next_col = (col + 1) % n;

        for num in 1..=(n * n) as i8 {
            if !used[num as usize] {
                let num_i32 = num as i32;

                // Early pruning based on row and column sums
                if col == n - 1 && row_sum + num_i32 != self.magic_sum {
                    continue;
                }
                if row == n - 1 && col_sum + num_i32 != self.magic_sum {
                    continue;
                }

                if square.set(row, col, num).is_ok() {
                    used[num as usize] = true;

                    let new_row_sum = if col == 0 { num_i32 } else { row_sum + num_i32 };
                    let new_col_sum = if row == 0 { num_i32 } else { col_sum + num_i32 };

                    if self.backtrack(square, next_row, next_col, used, new_row_sum, new_col_sum) {
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
