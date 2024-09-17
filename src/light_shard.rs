use crate::collider::Collider;
use crate::utility::Point;
use macroquad::prelude::*;

pub struct LightShard {
    size: f32,
    pub position: Point,
    pub is_collected: bool,
    pub collider: Collider,
    pub texture: Texture2D,
}

impl LightShard {
    const DEFAULT_SIZE: f32 = 20.0;

    pub async fn new(position: Point) -> Self {
        Self {
            size: Self::DEFAULT_SIZE,
            position,
            is_collected: false,
            collider: Collider {
                position,
                size: Self::DEFAULT_SIZE,
            },
            texture: load_texture("assets/textures/light-shard.png")
                .await
                .unwrap(),
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
                ..Default::default()
            },
        );
    }
}
