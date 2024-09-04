use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell{
    Dead,
    Alive
}

// In Grid I store cols and rows just for easiness, I could obtain them right from cells.
#[derive(Debug)]
struct Grid{
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl Grid{
    // New grid with dead cells
    fn new(rows: usize, cols: usize) -> Self {
        Grid { 
            cells: vec![vec![Cell::Dead; cols]; rows], 
            rows, cols
        }
    }

    // Example grid for testing
    fn new_preloaded() -> Self{
        Grid { cells: vec![
            vec![Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Alive, Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive],
        ], rows: 4, cols:4 }
    }

    // Gets neighbors of a given position and counts how many of them are alive cells.
    fn count_alive_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        let neighbors = self.get_neighbors(x, y);

        for neighbor in neighbors{
            if self.cells[neighbor.0][neighbor.1] == Cell::Alive {
                count += 1;
            }
        }
        
        count
    }

    // Get all neighbors of a given position (not the actual Cell velues, but their positions in the grid)
    // Handles the case in which there might be a position with fewer neighbors. Like [0,0]
    fn get_neighbors(&self, x:usize, y:usize) -> Vec<(usize, usize)>{
        let mut neighbors: Vec<(usize, usize)> = Vec::new();

        // Relative positions
        let deltas = [
            (-1, -1), (-1, 0), (-1, 1), 
            (0, -1), /*(0,0),*/ (0, 1), 
            (1, -1), (1, 0), (1, 1),
        ];

        for (dx, dy) in &deltas {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            // Check if the new position is a valid one
            if new_x >= 0 && new_x < self.rows as isize && 
                new_y >= 0 && new_y < self.cols as isize {
                neighbors.push((new_x as usize, new_y as usize));
            }
        }

        neighbors
    }

    // Calculates next generation and updates the grid with that result.
    fn update(&mut self) -> (){
        self.cells = self.calculate_next_gen();
    }

    // For each cell in the grid calculates if in the next gen is going to be alive or not. Returns the next generation.
    fn calculate_next_gen(&self) -> Vec<Vec<Cell>> {
        let mut next_gen: Vec<Vec<Cell>> = vec![vec![Cell::Dead; self.cols]; self.rows];

        for i in 0..self.rows{
            for j in 0..self.cols{
                let current_gen_cell = self.cells[i][j];
                let alive_neighbors = self.count_alive_neighbors(i, j);
                let next_gen_cell = match current_gen_cell{
                    Cell::Dead => 
                        if alive_neighbors == 3 { Cell::Alive } 
                            else {Cell::Dead},
                    Cell::Alive => 
                        if alive_neighbors == 2 || alive_neighbors == 3 {Cell::Alive} 
                            else {Cell::Dead},
                };
                next_gen[i][j] = next_gen_cell;
            }
        }

        next_gen
    }
}

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    clear_background(BLACK);

    let mut grid = Grid::new_preloaded(); // This is an example grid
    loop {
        // Here I should draw the grid on screen.

        grid.update();

        next_frame().await
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_initialization() {
        let grid = Grid::new(4, 4);
        for row in &grid.cells {
            for &cell in row {
                assert_eq!(cell, Cell::Dead);
            }
        }
    }

    #[test]
    fn test_preloaded_grid() {
        let grid = Grid::new_preloaded();
        assert_eq!(grid.cells[0][1], Cell::Alive);
        assert_eq!(grid.cells[1][0], Cell::Alive);
        assert_eq!(grid.cells[2][2], Cell::Alive);
        assert_eq!(grid.cells[3][3], Cell::Alive);
    }

    #[test]
    fn test_count_alive_neighbors() {
        let grid = Grid::new_preloaded();
        // Test the neighbors alive count for the cell at (1,1)
        let alive_neighbors = grid.count_alive_neighbors(1, 1);
        assert_eq!(alive_neighbors, 3);
    }

    #[test]
    fn test_get_neighbors() {
        let grid = Grid::new(4, 4);
        let neighbors = grid.get_neighbors(1, 1);

        // Check if all expected neighbors are present
        assert_eq!(neighbors, vec![
            (0, 0), (0, 1), (0, 2),
            (1, 0),         (1, 2),
            (2, 0), (2, 1), (2, 2),
        ]);
    }

    #[test]
    fn test_count_alive_neighbors_edge_case() {
        let grid = Grid::new_preloaded();

        let alive_neighbors = grid.count_alive_neighbors(0, 0);
        assert_eq!(alive_neighbors, 3);
    }


    // This tests if the next 
    #[test]
    fn test_next_generations() {
        let mut grid = Grid::new_preloaded();

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
}
