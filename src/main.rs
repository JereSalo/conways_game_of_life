

use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell{
    Dead,
    Alive
}

#[derive(Debug)]
struct Grid{
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl Default for Grid{
    fn default() -> Self {
        Grid { cells: vec![
            vec![Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Alive, Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive],
        ], rows: 4, cols:4 }
    }
}

impl Grid{
    fn new(rows: usize, cols: usize, default_value: Cell) -> Self {
        Grid { 
            cells: vec![vec![default_value; cols]; rows], 
            rows, cols
        }
    }

    fn new_preloaded() -> Self{
        Self::default()
    }

    // Gets neighbors of a given position and counts how many of them are alive cells.
    fn neighbors_alive(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        let neighbors = self.get_neighbors(x, y);

        for neighbor in neighbors{
            if self.cell_alive(neighbor.0, neighbor.1) {
                count += 1;
            }
        }
        
        count
    }

    // Get all neighbors of a given position
    // Handles the case in which there might be a position with fewer neighbors. 
    //      e.g: [0,0]
    fn get_neighbors(&self, x:usize, y:usize) -> Vec<(usize, usize)>{
        let mut neighbors = Vec::new();

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

    fn cell_alive(&self, x: usize, y: usize) -> bool {
        self.cells[x][y] == Cell::Alive
    }

    fn update(&mut self) -> (){
        self.cells = self.calculate_next_gen();
    }

    fn calculate_next_gen(&self) -> Vec<Vec<Cell>> {
        let mut ng: Vec<Vec<Cell>> = vec![vec![Cell::Dead; self.cols]; self.rows];

        //TODO: Check if it is first rows then cols or the other way around
        for i in 0..self.rows{
            for j in 0..self.cols{
                let current_gen_cell = self.cells[i][j];
                let alive_neighbors = self.neighbors_alive(i, j);
                let next_gen_cell = match current_gen_cell{
                    Cell::Dead => 
                        if alive_neighbors == 3 { Cell::Alive } 
                            else {Cell::Dead},
                    Cell::Alive => 
                        if alive_neighbors == 2 || alive_neighbors == 3 {Cell::Alive} 
                            else {Cell::Dead},
                };
                ng[i][j] = next_gen_cell;
            }
        }

        ng
    }


}

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    clear_background(BLACK);

    let mut grid = Grid::new_preloaded();
    loop {

        // So here I should update the grid I guess
        
        
        // println!("{:?}", grid);
        // grid.update();
        
        // println!("{}",a);
        // break;

        // 

        // 



        next_frame().await
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_initialization() {
        let grid = Grid::new(4, 4, Cell::Dead);
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
    fn test_neighbors_alive() {
        let grid = Grid::new_preloaded();
        // Test the neighbors alive count for the cell at (1,1)
        let alive_neighbors = grid.neighbors_alive(1, 1);
        assert_eq!(alive_neighbors, 3);
    }

    #[test]
    fn test_cell_alive() {
        let grid = Grid::new_preloaded();
        assert!(grid.cell_alive(1, 1));  // This should be alive
        assert!(!grid.cell_alive(0, 0)); // This should be dead
    }

    #[test]
    fn test_get_neighbors() {
        let grid = Grid::new(4, 4, Cell::Dead);
        let neighbors = grid.get_neighbors(1, 1);

        // Check if all expected neighbors are present
        assert_eq!(neighbors, vec![
            (0, 0), (0, 1), (0, 2),
            (1, 0),         (1, 2),
            (2, 0), (2, 1), (2, 2),
        ]);
    }

    #[test]
    fn test_neighbors_alive_edge_case() {
        let grid = Grid::new_preloaded();

        let alive_neighbors = grid.neighbors_alive(0, 0);
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
