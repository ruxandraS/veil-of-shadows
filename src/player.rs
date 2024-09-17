use crate::collider::Collider;
use crate::utility::Point;

use macroquad::prelude::*;

pub struct Player {
    size: f32,
    pub position: Point,
    pub collider: Collider,
}

impl Player {
    const DEFAULT_SIZE: f32 = 25.0;

    pub fn new() -> Self {
        let position = Point {
            x: Self::DEFAULT_SIZE / 2.0,
            y: screen_height() - (Self::DEFAULT_SIZE * 2.0),
        };

        Self {
            size: Self::DEFAULT_SIZE,
            position,
            collider: Collider {
                position,
                size: Self::DEFAULT_SIZE,
            },
        }
    }

    pub fn update_position(&mut self) {
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.position.x += 1.0;
            self.collider.position.x += 1.0;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.position.x -= 1.0;
            self.collider.position.x -= 1.0;
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
