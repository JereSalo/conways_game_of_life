use std::ops::Not;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Not for Cell {
    type Output = Cell;

    fn not(self) -> Cell {
        match self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}
