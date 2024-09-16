use macroquad::prelude::*;

#[macroquad::main("Veil of Shadows")]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        draw_text("Hello, welcome to Veil of Shadows!", 20.0, 20.0, 30.0, BEIGE);

        next_frame().await
    }
}
