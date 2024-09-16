use crate::utility::Point;
use macroquad::prelude::*;

pub struct LightShard {
    size: u8,
    pub position: Point,
    pub is_collected: bool,
}

impl LightShard {
    const DEFAULT_SIZE: u8 = 10;

    pub fn new(position: Point) -> Self {
        Self {
            size: Self::DEFAULT_SIZE,
            position,
            is_collected: false,
        }
    }

    pub fn render(&self) {
        if self.is_collected {
            return;
        }

        draw_circle(self.position.x, self.position.y, self.size.into(), YELLOW);
    }
}
