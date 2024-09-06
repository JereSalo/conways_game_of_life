use std::process;
use std::time::Duration;

use macroquad::prelude::*;

use crate::cell::Cell;
use crate::grid::Grid;

const ROWS: usize = 50;
const COLS: usize = 50;

/// UI representation
/// ## Attributes
/// - grid: Stores the state of cells
/// - paused: True if game is paused
/// - show_info: If true, shows info in UI
/// - scr_w: Screen Width
/// - scr_h: Screen Height
/// - cell_w: Cell Width
/// - cell_h: Cell Height\
/// Last 4 attributes are necessary cause window can be resized during execution.
pub struct UI {
    grid: Grid,
    paused: bool,
    show_info: bool,
    scr_w: f32,
    scr_h: f32,
    cell_w: f32,
    cell_h: f32,
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}

impl UI {
    /// Runs the game
    pub async fn run(&mut self) {
        loop {
            clear_background(BLACK);

            // Width and Height are set on every tick cause screen can be resized.
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

    /// Creates new UI
    pub fn new() -> Self {
        UI {
            grid: Grid::new(ROWS, COLS),
            paused: true,
            show_info: true,
            scr_w: 0.,
            scr_h: 0.,
            cell_w: 0.,
            cell_h: 0.,
        }
    }

    /// Draws the physical board in the UI
    fn draw_board(&self) {
        for i in 0..self.grid.rows() {
            for j in 0..self.grid.cols() {
                if self.grid.cells[i][j] == Cell::Alive {
                    draw_rectangle(
                        self.cell_w * j as f32,
                        self.cell_h * i as f32,
                        self.cell_w,
                        self.cell_h,
                        WHITE,
                    );
                }
            }
            // Draw vertical and horizontal lines of grid
            draw_line(
                0.0,
                self.cell_h * i as f32,
                self.scr_w,
                self.cell_h * i as f32,
                1.0,
                GRAY,
            );
            draw_line(
                self.cell_w * i as f32,
                0.0,
                self.cell_w * i as f32,
                self.scr_h,
                1.0,
                GRAY,
            );
        }
    }

    /// Gets screen size and based on that calculates cell size for drawing them in screen.
    fn get_screen_and_cell_size(&mut self) {
        self.scr_w = screen_width();
        self.scr_h = screen_height();
        self.cell_w = self.scr_w / self.grid.cols() as f32;
        self.cell_h = self.scr_h / self.grid.rows() as f32;
    }

    /// Takes action when user presses certain keys
    fn handle_key_press(&mut self) {
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

    /// Show information of key bindings on display
    fn show_info(&mut self) {
        let info = "'I' to toggle info on/off\n\
        'R' to generate random grid\n\
        'C' to clear the grid\n\
        'Space' to play/pause\n\
        'Click' to kill/revive cells\n\
        'Q' to quit the game";
        // Show key bindings

        draw_rectangle(
            self.scr_w * 0.2,
            self.scr_h * 0.7,
            self.scr_w * 0.6,
            self.scr_h * 0.26,
            BLACK,
        );
        draw_multiline_text(
            info,
            self.scr_w * 0.25,
            self.scr_h * 0.75,
            self.scr_h * 0.034,
            Some(self.scr_h / 800.),
            WHITE,
        );
    }

    /// Allows user to toggle the state of cells in the grid with left mouse button
    fn handle_cell_interaction(&mut self) {
        if is_mouse_button_released(MouseButton::Left) {
            let (x, y) = mouse_position();

            let i = (y / self.cell_h) as usize;
            let j = (x / self.cell_w) as usize;

            self.grid.cells[i][j] = !self.grid.cells[i][j];
        }
    }
}
