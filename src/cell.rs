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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_negation() {
        let alive_cell = Cell::Alive;
        let dead_cell = Cell::Dead;

        assert_eq!(!alive_cell, Cell::Dead);
        assert_eq!(!dead_cell, Cell::Alive);
    }
}
