use std::ops::Not;
/// Represents state of elements in grid\
/// Implements "Not" trait
/// ## Example
/// ```
/// use conways::cell::Cell;
/// let alive_cell = !Cell::Dead;
/// assert_eq!(alive_cell == Cell::Alive, true);
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
