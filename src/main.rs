

use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
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

    fn neighbors(x: u64, y: u64) -> Vec<Cell> {
        todo!();
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
        break;

        // 



        next_frame().await
    }
}
