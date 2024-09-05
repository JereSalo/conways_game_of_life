use macroquad::prelude::*;

use conways::cell::Cell;
use conways::grid::Grid;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut grid = Grid::new_random(200, 200);

    loop {
        clear_background(BLACK);
        // screen_width() and screen_height() to get it in real time. Measured in pixels. Default is 800x600
        let cell_w = screen_width() / grid.cols() as f32;
        let cell_h = screen_height() / grid.rows() as f32;

        #[allow(clippy::needless_range_loop)]
        for i in 0..grid.rows() {
            for j in 0..grid.cols() {
                if grid.cells[i][j] == Cell::Alive {
                    draw_rectangle(cell_w * j as f32, cell_h * i as f32, cell_w, cell_h, WHITE);
                }
            }
        }

        grid.update();

        next_frame().await
    }
}
