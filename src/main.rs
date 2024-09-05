use std::time::Duration;

use macroquad::prelude::*;

use conways::cell::Cell;
use conways::grid::Grid;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let (rows, cols) = (50, 50);
    let mut grid = Grid::new(rows, cols);
    let mut paused = true;
    let mut show_info = true;

    loop {
        // If it is not paused
        clear_background(BLACK);

        // Width and Height are defined on every tick cause screen can be resized.
        let scr_w = screen_width();
        let scr_h = screen_height();
        let cell_w = scr_w / grid.cols() as f32;
        let cell_h = scr_h / grid.rows() as f32;

        // Draw Board
        #[allow(clippy::needless_range_loop)]
        for i in 0..grid.rows() {
            for j in 0..grid.cols() {
                if grid.cells[i][j] == Cell::Alive {
                    draw_rectangle(cell_w * j as f32, cell_h * i as f32, cell_w, cell_h, WHITE);
                }
            }
            // Draw vertical an horizontal lines of grid
            draw_line(0.0, cell_h * i as f32, scr_w, cell_h * i as f32, 1.0, GRAY);
            draw_line(cell_w * i as f32, 0.0, cell_w * i as f32, scr_h, 1.0, GRAY);
        }

        // Handle KeyPress: R (Random), C (Clear), Space (Play/Pause), I (Toggle Info)
        match get_last_key_pressed() {
            Some(KeyCode::R) => grid = Grid::new_random(rows, cols),
            Some(KeyCode::C) => grid = Grid::new(rows, cols),
            Some(KeyCode::Space) => paused = !paused,
            Some(KeyCode::I) => show_info = !show_info,
            Some(_) | None => {}
        }

        if show_info {
            let info = "'I' to toggle info on/off\n\
            'R' to generate random grid\n\
            'C' to clear the grid\n\
            'Space' to play/pause\n\
            'Click' to kill/revive cells";
            // Show key bindings

            draw_rectangle(scr_w * 0.2, scr_h * 0.7, scr_w * 0.6, scr_h * 0.26, BLACK);
            draw_multiline_text(
                info,
                scr_w * 0.25,
                scr_h * 0.75,
                scr_h * 0.038,
                Some(scr_h / 800.),
                WHITE,
            );
        }

        if !paused {
            grid.update();
            std::thread::sleep(Duration::from_millis(40));
        } else {
            // Just to show that it's paused without being invasive.
            draw_hexagon(20.0, 20.0, 15.0, 2.0, true, RED, RED);

            // Let user kill/revive cells
            if is_mouse_button_released(MouseButton::Left) {
                let (x, y) = mouse_position();

                let i = (y / cell_h) as usize;
                let j = (x / cell_w) as usize;

                grid.cells[i][j] = !grid.cells[i][j];
            }
        }

        next_frame().await
    }
}
