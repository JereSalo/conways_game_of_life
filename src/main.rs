use std::time::Duration;

use macroquad::prelude::*;

use conways::cell::Cell;
use conways::grid::Grid;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut grid = Grid::new_random(50, 50);
    let mut paused = false;

    loop {
        // If it is not paused 
        if !paused{
            grid.update();
        }

        // Cell width and height. I define these in every tick cause screen can be resized.
        let cell_w = screen_width() / grid.cols() as f32;
        let cell_h = screen_height() / grid.rows() as f32;

        // Draw Board
        #[allow(clippy::needless_range_loop)]
        for i in 0..grid.rows() {
            for j in 0..grid.cols() {
                if grid.cells[i][j] == Cell::Alive {
                    draw_rectangle(cell_w * j as f32, cell_h * i as f32, cell_w, cell_h, WHITE);
                }
            }
            draw_line(0.0, cell_h * i as f32, screen_width(), cell_h * i as f32, 1.0, GRAY);
            draw_line(cell_w*i as f32, 0.0, cell_w*i as f32, screen_height(), 1.0, GRAY);
        }

        // Handle KeyPress: R (Random), C (Clear), Space (Play/Pause)
        match get_last_key_pressed(){
            Some(KeyCode::R) => grid = Grid::new_random(50, 50),
            Some(KeyCode::C) => grid = Grid::new(50, 50),
            Some(KeyCode::Space) => paused = !paused,
            Some(_) | None => {}
        }

        std::thread::sleep(Duration::from_millis(40));

        next_frame().await
    }
}
