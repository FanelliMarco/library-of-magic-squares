use std::fmt;

#[derive(Debug)]
pub struct MagicSquare {
    square: Vec<i8>,
    order: usize,
}

impl MagicSquare {
    pub fn new(order: usize) -> MagicSquare {
        MagicSquare {
            square: (1..=i8::try_from(order.pow(2)).unwrap_or(i8::MAX)).collect(),
            order,
        }
    }

    // Function to convert square Vec<i8> to Vec<Vec<i8>>
    pub fn to_2d_vec(&self) -> Vec<Vec<i8>> {
        self.square
            .chunks(self.order)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}

impl fmt::Display for MagicSquare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.to_2d_vec() {
            for &num in &row {
                write!(f, "{:4}", num)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
