

use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell{
    Dead,
    Alive
}

impl Cell{
    // Gets alive neighbors of the cell (it's a number between 0 and 8)
    fn get_alive_neighbors(&self) -> u8 {
        todo!();
    }

    fn next_state(&self) -> Self{
        todo!("")
    }
}

#[derive(Debug)]
struct Grid{
    cells: Vec<Vec<Cell>>,
    next_gen: Vec<Vec<Cell>>,
}

impl Default for Grid{
    fn default() -> Self {
        Grid { cells: vec![
            vec![Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Alive, Cell::Alive, Cell::Dead, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead],
            vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive],
        ], next_gen: vec![vec![Cell::Dead; 4];4] }
    }
}

impl Grid{
    fn new(rows: usize, cols: usize, default_value: Cell) -> Self {
        Grid { 
            cells: vec![vec![default_value; cols]; rows], 
            next_gen: vec![vec![Cell::Dead; cols];rows]
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

    fn get_neighbors(&self, x:usize, y:usize) -> Vec<(usize, usize)>{
        // Need to handle the case in which the position may not have some neighbors
        vec![(x-1,y-1), (x-1, y), (x-1, y+1), 
            (x, y-1), (x, y+1), 
            (x+1, y-1), (x+1, y), (x+1, y+1)]
    }

    fn cell_alive(&self, x: usize, y: usize) -> bool {
        self.cells[x][y] == Cell::Alive
    }


}

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    clear_background(BLACK);
    let rows = 4;
    let cols = 4;

    let grid = Grid::new_preloaded();
    loop {

        

        println!("{:?}", grid);
        // let a = grid.neighbors_alive(2, 2);
        // println!("{}",a);
        break;

        // 



        next_frame().await
    }
}
