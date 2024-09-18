use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;
use player::Player;
use world::World;

mod animation;
mod collider;
mod light_shard;
mod platform;
mod player;
mod utility;
mod world;

#[macroquad::main("Veil of Shadows")]
async fn main() {
    let canvas = Canvas2D::new(screen_width(), screen_height());

    let mut player = Player::new().await;
    let mut world = World::new().await;

    loop {
        let delta = get_frame_time();

        player.update_position(delta);
        world.update(delta);
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
