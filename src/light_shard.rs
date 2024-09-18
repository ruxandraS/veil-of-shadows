use crate::{animation::Animation, collider::Collider, utility::Point};
use macroquad::prelude::*;

pub struct LightShard {
    size: f32,
    pub position: Point,
    pub is_collected: bool,
    pub collider: Collider,
    pub texture: Texture2D,
    pub animation: Animation,
}

impl LightShard {
    const DEFAULT_SIZE: f32 = 20.0;

    pub async fn new(position: Point) -> Self {
        Self {
            size: Self::DEFAULT_SIZE,
            position,
            is_collected: false,
            collider: Collider::square(position, Self::DEFAULT_SIZE),
            texture: {
                let texture = load_texture("assets/textures/light-shard.png")
                    .await
                    .unwrap();
                texture.set_filter(FilterMode::Nearest);
                texture
            },
            animation: Animation::new(16, 0, 1.0 / 2.0),
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.animation.time += delta;

        if self.is_collected {
            return;
        }

        if self.animation.time >= self.animation.speed {
            self.animation.rotate();
            self.animation.time = 0.0;
        }
    }

    pub fn render(&self) {
        if self.is_collected {
            return;
        }

        draw_texture_ex(
            &self.texture,
            self.position.x - self.size / 2.0,
            self.position.y - self.size / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.size, self.size)),
                rotation: self.animation.rotation,
                ..Default::default()
            },
        );
    }
}
