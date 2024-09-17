use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;
use player::Player;
use world::World;

mod collider;
mod light_shard;
mod player;
mod utility;
mod world;

#[macroquad::main("Veil of Shadows")]
async fn main() {
    let canvas = Canvas2D::new(screen_width(), screen_height());

    let mut player = Player::new();
    let mut world = World::new().await;

    loop {
        player.update_position();
        world.check_collisions(&player.collider);

        set_camera(&canvas.camera);
        clear_background(DARKGRAY);

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
