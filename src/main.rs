use std::time::Duration;

use macroquad::prelude::*;

use conways::cell::Cell;
use conways::grid::Grid;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut grid = Grid::new_random(50, 50);

    loop {
        // Handle KeyPress: R (Reset), Space (Play/Pause)
        // Reset: Create random grid
        match get_char_pressed() {
            // Reset, by now, generates a random grid. Maybe it will just generate a clear grid in the future.
            Some('r' | 'R') => grid = Grid::new_random(50, 50),
            Some(' ') => println!("Space: Play/Pause"),
            Some(c) => println!("You pressed another key: {}", c),
            None => {}
        }

        clear_background(BLACK);
        
        // Cell width and height. I define these in every tick cause screen can be resized.
        let cell_w = screen_width() / grid.cols() as f32;
        let cell_h = screen_height() / grid.rows() as f32;

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

        grid.update();

        std::thread::sleep(Duration::from_millis(50));

        next_frame().await
    }
}
