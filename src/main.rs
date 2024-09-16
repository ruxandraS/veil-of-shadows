use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;
use player::Player;

mod player;
mod utility;

#[macroquad::main("Veil of Shadows")]
async fn main() {
    let mut player = Player::new();
    let canvas = Canvas2D::new(screen_width(), screen_height());

    loop {
        set_camera(&canvas.camera);
        clear_background(DARKGRAY);

        player.update_position();
        player.render();

        // Reset the camera
        set_default_camera();

        canvas.draw();

        // Game UI
        draw_text(
            "Hello, welcome to Veil of Shadows!",
            20.0,
            20.0,
            30.0,
            BEIGE,
        );

        next_frame().await
    }
}
