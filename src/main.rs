use macroquad::prelude::*;

use conways::grid::Grid;
use conways::cell::Cell;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    clear_background(BLACK);

    let mut grid = Grid::new_from_cells(vec![
        vec![Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
        vec![Cell::Alive, Cell::Alive, Cell::Dead, Cell::Dead],
        vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead],
        vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive],
    ]); // This is an example grid, for testing purposes only

    loop {
        // Here I should draw the grid state on screen I guess.

        grid.update();

        next_frame().await
    }
}
