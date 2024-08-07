use std::fmt;

#[derive(Debug, Clone)]
pub struct MagicSquare {
    square: Vec<i8>,
    order: usize,
}

impl MagicSquare {
    pub fn new(order: usize) -> Self {
        MagicSquare {
            square: vec![0; order * order],
            order,
        }
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn get(&self, row: usize, col: usize) -> Option<i8> {
        if row < self.order && col < self.order {
            Some(self.square[row * self.order + col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: i8) -> Result<(), &'static str> {
        if row < self.order && col < self.order {
            self.square[row * self.order + col] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }

    pub fn to_2d_vec(&self) -> Vec<Vec<i8>> {
        self.square
            .chunks(self.order)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    pub fn magic_constant(&self) -> usize {
        self.order * (self.order * self.order + 1) / 2
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
