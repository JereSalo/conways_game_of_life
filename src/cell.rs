use std::ops::Not;
/// Represents state of elements in grid\
/// Implements "Not" trait
/// ## Example
/// ```
/// let alive_cell = !Cell::Dead;
/// return alive_cell == Cell::Alive;
/// ```
/// `returns true`
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
