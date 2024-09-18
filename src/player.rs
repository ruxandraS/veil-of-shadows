use std::collections::HashMap;

use crate::{animation::Animation, collider::Collider, utility::Point};

use macroquad::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum State {
    Idle,
    Running,
}

pub struct Player {
    size: f32,
    pub position: Point,
    pub collider: Collider,
    pub state: State,
    pub texture_map: HashMap<State, Texture2D>,
    pub animation: Animation,
}

impl Player {
    const DEFAULT_SIZE: f32 = 25.0;

    pub async fn new() -> Self {
        let texture_map: HashMap<State, Texture2D> = [
            (State::Idle, {
                let texture = load_texture("assets/textures/player/Meow-Knight_Idle.png")
                    .await
                    .unwrap();
                texture.set_filter(FilterMode::Nearest);
                texture
            }),
            (State::Running, {
                let texture = load_texture("assets/textures/player/Meow-Knight_Run.png")
                    .await
                    .unwrap();
                texture.set_filter(FilterMode::Nearest);
                texture
            }),
        ]
        .into_iter()
        .collect();

        let position = Point {
            x: Self::DEFAULT_SIZE / 2.0,
            y: screen_height() - (Self::DEFAULT_SIZE * 2.0),
        };

        Self {
            size: Self::DEFAULT_SIZE,
            position,
            collider: Collider::square(position, Self::DEFAULT_SIZE),
            state: State::Idle,
            texture_map,
            animation: Animation::new(16, 6, 1.0 / 24.0),
        }
    }

    pub fn update_position(&mut self, delta: f32) {
        let previous_state = self.state;
        self.animation.time += delta;

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.state = State::Running;

            self.position.x += 1.0;
            self.collider.position.x += 1.0;
            self.animation.flip = false;
        } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.state = State::Running;

            self.position.x -= 1.0;
            self.collider.position.x -= 1.0;
            self.animation.flip = true;
        } else {
            self.state = State::Idle;
        }

        if self.state != previous_state {
            self.animation.reset_frame();
        } else if self.animation.time >= self.animation.speed {
            self.animation.next_frame();
        }
    }

    pub fn render(&self) {
        draw_texture_ex(
            &self.texture_map[&self.state],
            self.position.x - self.size / 2.0,
            self.position.y - self.size / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.size, self.size)),
                source: Some(Rect::new(
                    0.0,
                    (self.animation.current_frame * self.animation.frame_size).into(),
                    self.animation.frame_size.into(),
                    self.animation.frame_size.into(),
                )),
                flip_x: self.animation.flip,
                ..Default::default()
            },
        );

        // draw_rectangle(
        //     self.position.x,
        //     self.position.y,
        //     self.size.into(),
        //     self.size.into(),
        //     GREEN,
        // );
    }
}
