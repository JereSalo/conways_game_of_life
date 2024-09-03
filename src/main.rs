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
        //  Attributes: Position on a grid, a state (dead or alive)
        //  Methods: next_state (calculates how many neighbors alive and decides whether or not it lives or dies)

        // Grid
        //  

        next_frame().await
    }
}
