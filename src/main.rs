use std::time::Duration;

use macroquad::prelude::*;

use conways::cell::Cell;
use conways::grid::Grid;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut grid = Grid::new_random(50, 50);
    let mut paused = false;
    let mut show_info = true;

    loop {
        // If it is not paused 
        clear_background(BLACK);

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
            Some(KeyCode::I) => show_info = !show_info,
            Some(_) | None => {}
        }

        if show_info{
            let info = "'I' to toggle info on/off\n\
            'R' to generate random grid\n\
            'C' to clear the grid\n\
            'Space' to play/pause\n\
            'Click' to kill/revive cells (TBD)";
            // Show key bindings
            let w = screen_width();
            let h = screen_height();
            draw_rectangle(w*0.2, h*0.7, w*0.6, h*0.26, BLACK);
            draw_multiline_text(info, w* 0.25, h* 0.75, h*0.038, Some(h / 800.), WHITE);
        }

        if !paused{
            grid.update();
        } else{
            // Let user kill/revive cells
            
        }

        std::thread::sleep(Duration::from_millis(40));

        next_frame().await
    }
}
