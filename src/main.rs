use std::thread::sleep;
use std::time::Duration;

use macroquad::prelude::*;

use conways::cell::{self, Cell};
use conways::grid::Grid;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    clear_background(RED);

    let mut grid = Grid::new_from_cells(vec![
        vec![Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
        vec![Cell::Alive, Cell::Alive, Cell::Dead, Cell::Dead],
        vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead],
        vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive],
    ]); // This is an example grid, for testing purposes only

    
    loop {
        // screen_width() and screen_height() to get it in real time. Measured in pixels. Default is 800x600
        let cell_w = screen_width() / grid.cols() as f32;
        let cell_h = screen_height() / grid.rows() as f32;

        for i in 0..grid.rows(){
            for j in 0..grid.cols(){
                if grid.cells[i][j] == Cell::Alive{
                    draw_rectangle(cell_w * j as f32, cell_h * i as f32, cell_w, cell_h, WHITE);
                }
                else{
                    draw_rectangle(cell_w * j as f32, cell_h * i as f32, cell_w, cell_h, BLACK);
                }
            }
        }

        // sleep(Duration::from_secs(1)); // For testing purposes
        
        grid.update();
        

        next_frame().await
    }
}
