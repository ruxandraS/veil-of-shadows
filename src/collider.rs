use macroquad::math::Rect;

use crate::utility::Point;

pub struct Collider {
    pub position: Point,
    pub size: f32,
    pub height: Option<f32>,
}

impl Collider {
    pub fn square(position: Point, size: f32) -> Self {
        Self {
            position,
            size,
            height: None,
        }
    }

    pub fn rectangle(position: Point, size: f32, height: f32) -> Self {
        Self {
            position,
            size,
            height: Some(height),
        }
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.position.x - self.size / 2.0,
            y: self.position.y - self.height.unwrap_or(self.size / 2.0),
            w: self.size,
            h: self.height.unwrap_or(self.size),
        }
    }
}
