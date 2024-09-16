use macroquad::prelude::*;
use player::Player;

mod player;
mod utility;

#[macroquad::main("Veil of Shadows")]
async fn main() {
    let mut player = Player::new();

    loop {
        clear_background(DARKGRAY);

        draw_text(
            "Hello, welcome to Veil of Shadows!",
            20.0,
            20.0,
            30.0,
            BEIGE,
        );

        player.update_position();
        player.render();

        next_frame().await
    }
}
