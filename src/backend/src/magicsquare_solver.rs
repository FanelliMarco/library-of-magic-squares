// File: src/backend/src/magicsquare_solver.rs

use crate::magicsquare::MagicSquare;

pub struct MagicSquareSolver;

impl MagicSquareSolver {
    pub fn solve(magic_square: &mut MagicSquare) -> bool {
        let n = magic_square.order;
        let target_sum = (n * (n * n + 1) / 2) as i32;
        let mut used = vec![false; (n * n + 1) as usize];

        fn backtrack(
            square: &mut Vec<i8>,
            n: usize,
            index: usize,
            used: &mut Vec<bool>,
            target_sum: i32,
        ) -> bool {
            if index == n * n {
                return is_magic_square(square, n, target_sum);
            }

            for num in 1..=(n * n) as i8 {
                if !used[num as usize] {
                    square[index] = num;
                    used[num as usize] = true;

                    if backtrack(square, n, index + 1, used, target_sum) {
                        return true;
                    }

                    used[num as usize] = false;
                    square[index] = 0;
                }
            }

            false
        }

        backtrack(&mut magic_square.square, n, 0, &mut used, target_sum)
    }
}

fn is_magic_square(square: &Vec<i8>, n: usize, target_sum: i32) -> bool {
    // Check rows
    for i in 0..n {
        if square[i * n..(i + 1) * n]
            .iter()
            .map(|&x| x as i32)
            .sum::<i32>()
            != target_sum
        {
            return false;
        }
    }

    // Check columns
    for i in 0..n {
        if (0..n).map(|j| square[j * n + i] as i32).sum::<i32>() != target_sum {
            return false;
        }
    }

    // Check main diagonal
    if (0..n).map(|i| square[i * n + i] as i32).sum::<i32>() != target_sum {
        return false;
    }

    // Check anti-diagonal
    if (0..n)
        .map(|i| square[i * n + (n - 1 - i)] as i32)
        .sum::<i32>()
        != target_sum
    {
        return false;
    }

    true
}
