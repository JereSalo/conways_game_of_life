use std::arch::aarch64::int16x4_t;

use macroquad::prelude::*;

enum Cell{
    Dead,
    Alive
}

impl Cell{

    // Gets alive neighbors of the cell (it's a number between 0 and 8)
    fn get_alive_neighbors(&self) -> u8 {
        todo!();
    }
}


#[macroquad::main("Conway's Game of Life")]
async fn main() {
    loop {
        clear_background(BLACK);

        // Cell: Enum?
        //  Methods: next_state (calculates how many neighbors alive and decides whether or not it lives or dies)

        // Grid: 100x100 grid? From [0][0] to [99][99]
        //  It has cells in it, on each space available there is a cell
        //  You can ask to it for the neighbors of a cell and it will return 8 cells (?)

        next_frame().await
    }
}
