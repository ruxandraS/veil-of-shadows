use macroquad::math::Rect;

use crate::utility::Point;

pub struct Collider {
    pub position: Point,
    pub size: f32,
}

impl Collider {
    pub fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.position.x - self.size / 2.0,
            y: self.position.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}
