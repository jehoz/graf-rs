use macroquad::prelude::*;

#[macroquad::main("GRAF")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_text("G R A f", 20.0, 20.0, 30.0, WHITE);

        next_frame().await
    }
}
