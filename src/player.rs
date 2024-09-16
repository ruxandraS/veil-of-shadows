use crate::utility::Point;
use macroquad::prelude::*;

pub struct Player {
    size: u8,
    pub position: Point,
}

impl Player {
    const DEFAULT_SIZE: u8 = 25;

    pub fn new() -> Self {
        Self {
            size: Self::DEFAULT_SIZE,
            position: Point {
                x: Self::DEFAULT_SIZE as f32 / 2.0,
                y: screen_height() - (Self::DEFAULT_SIZE as f32 * 2.0),
            },
        }
    }

    pub fn update_position(&mut self) {
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.position.x += 1.0;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.position.x -= 1.0;
        }
    }

    pub fn render(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.into(),
            self.size.into(),
            GREEN,
        );
    }
}
