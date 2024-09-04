use rand::prelude::*;

use crate::cell::Cell;

// Useful const for getting neighbors of cell.
const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn rows(&self) -> usize {
        self.cells.len()
    }

    fn cols(&self) -> usize {
        self.cells[0].len()
    }

    // New grid with dead cells
    pub fn new(rows: usize, cols: usize) -> Self {
        Grid {
            cells: vec![vec![Cell::Dead; cols]; rows],
        }
    }

    pub fn new_from_cells(cells: Vec<Vec<Cell>>) -> Self {
        Grid { cells }
    }

    pub fn new_random(rows: usize, cols: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = vec![vec![Cell::Dead; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                if rng.gen_bool(0.5) {
                    cells[i][j] = Cell::Alive;
                }
            }
        }

        Grid { cells }
    }

    // Gets neighbors of a given position and counts how many of them are alive cells.
    pub fn count_alive_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        let neighbors = self.get_neighbors(x, y);

        for neighbor in neighbors {
            if self.cells[neighbor.0][neighbor.1] == Cell::Alive {
                count += 1;
            }
        }

        count
    }

    // Get all neighbors of a given position (not the actual Cell velues, but their positions in the grid)
    // Handles the case in which there might be a position with fewer neighbors. Like [0,0]
    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();

        for (dx, dy) in DELTAS {
            // I use isize because result can be negative.
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            // Only push neighbor to neighbors if it's position is valid
            if new_x >= 0
                && new_x < self.rows() as isize
                && new_y >= 0
                && new_y < self.cols() as isize
            {
                neighbors.push((new_x as usize, new_y as usize));
            }
        }

        neighbors
    }

    // Calculates next generation and updates the grid with that result.
    pub fn update(&mut self) {
        self.cells = self.calculate_next_gen();
    }

    // For each cell in the grid calculates if in the next gen is going to be alive or not. Returns the next generation.
    pub fn calculate_next_gen(&self) -> Vec<Vec<Cell>> {
        let mut next_gen: Vec<Vec<Cell>> = vec![vec![Cell::Dead; self.cols()]; self.rows()];

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                let current_gen_cell = self.cells[i][j];
                let alive_neighbors = self.count_alive_neighbors(i, j);
                let next_gen_cell = match current_gen_cell {
                    Cell::Alive if alive_neighbors == 2 || alive_neighbors == 3 => Cell::Alive,
                    Cell::Dead if alive_neighbors == 3 => Cell::Alive,
                    _ => Cell::Dead,
                };
                next_gen[i][j] = next_gen_cell;
            }
        }

        next_gen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_preloaded_grid() -> Grid {
        Grid::new_from_cells(vec![
            vec![Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Alive, Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive],
        ])
    }

    #[test]
    fn test_grid_initialization() {
        let grid = Grid::new(4, 4);
        for row in grid.cells {
            for cell in row {
                assert_eq!(cell, Cell::Dead);
            }
        }
    }

    #[test]
    fn test_count_alive_neighbors() {
        let grid = create_preloaded_grid();
        // Test the neighbors alive count for the cell at (1,1)
        let alive_neighbors = grid.count_alive_neighbors(1, 1);
        assert_eq!(alive_neighbors, 3);
    }

    #[test]
    fn test_count_alive_neighbors_edge_case() {
        let grid = create_preloaded_grid();

        let alive_neighbors = grid.count_alive_neighbors(0, 0);
        assert_eq!(alive_neighbors, 3);
    }

    #[test]
    fn test_get_neighbors() {
        let grid = Grid::new(4, 4);
        let neighbors = grid.get_neighbors(1, 1);

        // Check if all expected neighbors are present
        assert_eq!(
            neighbors,
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2),
            ]
        );
    }

    // This tests if the next 2 generations are calculated correctly
    #[test]
    fn test_next_generations() {
        let mut grid = create_preloaded_grid();

        grid.update();
        let second_gen = grid.cells.clone();

        grid.update();
        let third_gen = grid.cells.clone();

        let expected_second_gen = vec![
            vec![Cell::Alive, Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Alive, Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Alive, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead],
        ];

        let expected_third_gen = vec![
            vec![Cell::Alive, Cell::Dead, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead],
            vec![Cell::Alive, Cell::Dead, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead],
        ];

        assert_eq!(*second_gen, expected_second_gen);

        assert_eq!(*third_gen, expected_third_gen);
    }

    // I tested it this way because possible combinations of a grid this size are 2^256, so we can be sure that there won't be a coincidence
    #[test]
    fn test_random() {
        let grid1 = Grid::new_random(16, 16);
        let grid2 = Grid::new_random(16, 16);

        assert_ne!(grid1.cells, grid2.cells);
    }
}
