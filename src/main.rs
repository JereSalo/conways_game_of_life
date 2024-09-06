use macroquad::prelude::*;

use conways::ui::UI;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut ui = UI::new();

    loop {
        ui.run();

        next_frame().await
    }
}
