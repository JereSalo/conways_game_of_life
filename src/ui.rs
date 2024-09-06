use std::process;
use std::time::Duration;

use macroquad::prelude::*;

use crate::cell::Cell;
use crate::grid::Grid;

const ROWS: usize = 50;
const COLS: usize = 50;

pub struct UI{
    grid: Grid,
    paused: bool,
    show_info: bool,
    scr_w: f32,
    scr_h: f32,
    cell_w: f32,
    cell_h: f32,
}

impl UI{
    pub async fn run(&mut self){
        loop {
            clear_background(BLACK);

            self.get_screen_and_cell_size();

            self.draw_board();

            self.handle_key_press();
            
            if self.show_info {
                self.show_info();
            }

            if !self.paused {
                self.grid.update();
                std::thread::sleep(Duration::from_millis(40));
            } else {
                // Just to show that it's paused without being invasive.
                draw_hexagon(20.0, 20.0, 15.0, 2.0, true, RED, RED);

                self.handle_cell_interaction();
            }
        
            next_frame().await;
        }
    }

    pub fn new() -> Self{
        UI {grid: Grid::new(ROWS,COLS),
             paused: true, 
             show_info: true, 
             scr_w: 0.,
             scr_h: 0.,
             cell_w: 0.,
             cell_h: 0.
        }
    }

    fn draw_board(&self){
          // Draw Board
          for i in 0..self.grid.rows() {
            for j in 0..self.grid.cols() {
                if self.grid.cells[i][j] == Cell::Alive {
                    draw_rectangle(self.cell_w * j as f32, self.cell_h * i as f32, self.cell_w, self.cell_h, WHITE);
                }
            }
            // Draw vertical an horizontal lines of grid
            draw_line(0.0, self.cell_h * i as f32, self.scr_w, self.cell_h * i as f32, 1.0, GRAY);
            draw_line(self.cell_w * i as f32, 0.0, self.cell_w * i as f32, self.scr_h, 1.0, GRAY);
        }
    }

    fn get_screen_and_cell_size(&mut self){
        // Width and Height are defined on every tick cause screen can be resized.
        self.scr_w = screen_width();
        self.scr_h = screen_height();
        self.cell_w = self.scr_w / self.grid.cols() as f32;
        self.cell_h = self.scr_h / self.grid.rows() as f32;
    }

    fn handle_key_press(&mut self){
        // Handle KeyPress: R (Random), C (Clear), Space (Play/Pause), I (Toggle Info), Q (Quit)
        match get_last_key_pressed() {
            Some(KeyCode::R) => self.grid = Grid::new_random(ROWS, COLS),
            Some(KeyCode::C) => self.grid = Grid::new(ROWS, COLS),
            Some(KeyCode::Space) => self.paused = !self.paused,
            Some(KeyCode::I) => self.show_info = !self.show_info,
            Some(KeyCode::Q) => process::exit(0),
            Some(_) | None => {}
        }
    }

    fn show_info(&mut self){
        let info = "'I' to toggle info on/off\n\
        'R' to generate random grid\n\
        'C' to clear the grid\n\
        'Space' to play/pause\n\
        'Click' to kill/revive cells\n\
        'Q' to quit the game";
        // Show key bindings

        draw_rectangle(self.scr_w * 0.2, self.scr_h * 0.7, self.scr_w * 0.6, self.scr_h * 0.26, BLACK);
        draw_multiline_text(
            info,
            self.scr_w * 0.25,
            self.scr_h * 0.75,
            self.scr_h * 0.034,
            Some(self.scr_h / 800.),
            WHITE,
        );
    }

    fn handle_cell_interaction(&mut self){
        // Let user kill/revive cells
        if is_mouse_button_released(MouseButton::Left) {
            let (x, y) = mouse_position();

            let i = (y / self.cell_h) as usize;
            let j = (x / self.cell_w) as usize;

            self.grid.cells[i][j] = !self.grid.cells[i][j];
        }
    }
}
