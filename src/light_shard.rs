use crate::collider::Collider;
use crate::utility::Point;
use macroquad::prelude::*;

pub struct LightShard {
    size: f32,
    pub position: Point,
    pub is_collected: bool,
    pub collider: Collider,
}

impl LightShard {
    const DEFAULT_SIZE: f32 = 10.0;

    pub fn new(position: Point) -> Self {
        Self {
            size: Self::DEFAULT_SIZE,
            position,
            is_collected: false,
            collider: Collider {
                position,
                size: Self::DEFAULT_SIZE,
            },
        }
    }

    pub fn render(&self) {
        if self.is_collected {
            return;
        }

        draw_circle(self.position.x, self.position.y, self.size.into(), YELLOW);
    }
}
