use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;
use player::Player;
use world::World;

mod light_shard;
mod player;
mod utility;
mod world;

#[macroquad::main("Veil of Shadows")]
async fn main() {
    let canvas = Canvas2D::new(screen_width(), screen_height());

    let mut player = Player::new();
    let world = World::new();

    loop {
        set_camera(&canvas.camera);
        clear_background(DARKGRAY);

        player.update_position();
        player.render();

        world.render();

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
