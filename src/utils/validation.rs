use crate::square::magic::MagicSquare;

pub fn is_magic_square(square: &MagicSquare) -> bool {
    let n = square.order();
    let target_sum = square.magic_constant() as i32;

    // Check rows and columns
    for i in 0..n {
        if (0..n)
            .map(|j| square.get(i, j).unwrap() as i32)
            .sum::<i32>()
            != target_sum
            || (0..n)
                .map(|j| square.get(j, i).unwrap() as i32)
                .sum::<i32>()
                != target_sum
        {
            return false;
        }
    }

    // Check main diagonal
    if (0..n)
        .map(|i| square.get(i, i).unwrap() as i32)
        .sum::<i32>()
        != target_sum
    {
        return false;
    }

    // Check anti-diagonal
    if (0..n)
        .map(|i| square.get(i, n - 1 - i).unwrap() as i32)
        .sum::<i32>()
        != target_sum
    {
        return false;
    }

    true
}
