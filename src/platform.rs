use crate::{animation::Animation, collider::Collider, utility::Point};
use macroquad::{color::GREEN, shapes::draw_rectangle};

#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    fn vector(&self) -> Point {
        match self {
            Self::Horizontal => Point { x: 1.0, y: 0.0 },
            Self::Vertical => Point { x: 0.0, y: 1.0 },
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum PlatformState {
    MovingToStart,
    MovingToEnd,
    StoppedStart,
    StoppedEnd,
}

#[derive(Debug)]
pub enum PlatformKind {
    Elevator {
        direction: Direction,
        state: PlatformState,
    },
    Raft {
        direction: Direction,
        state: PlatformState,
    },
}

impl PlatformKind {
    fn elevator() -> Self {
        Self::Elevator {
            direction: Direction::Vertical,
            state: PlatformState::StoppedEnd,
        }
    }

    fn raft() -> Self {
        Self::Raft {
            direction: Direction::Horizontal,
            state: PlatformState::StoppedStart,
        }
    }
}

pub struct Platform {
    pub kind: PlatformKind,
    pub position: Point,
    pub width: f32,
    pub height: f32,
    pub start: Point,
    pub end: Point,
    pub collider: Collider,
    pub animation: Animation,
}

impl Platform {
    fn new(
        kind: PlatformKind,
        position: Point,
        width: f32,
        height: f32,
        start: Point,
        end: Point,
    ) -> Self {
        Self {
            kind,
            position,
            width,
            height,
            start: position + start,
            end: position + end,
            collider: Collider::rectangle(position, width, height),
            animation: Animation::new(16, 0, 10.0),
        }
    }

    pub fn elevator(position: Point, width: f32, height: f32, start: Point, end: Point) -> Self {
        Self::new(
            PlatformKind::elevator(),
            position,
            width,
            height,
            start,
            end,
        )
    }

    pub fn raft(position: Point, width: f32, height: f32, start: Point, end: Point) -> Self {
        Self::new(PlatformKind::raft(), position, width, height, start, end)
    }

    pub fn update(&mut self, delta: f32) {
        self.animation.time += delta;

        let mut new_position;
        match &mut self.kind {
            PlatformKind::Elevator { state, direction }
            | PlatformKind::Raft { state, direction } => match *state {
                PlatformState::StoppedStart => {
                    if self.animation.time <= 2.0 {
                        return;
                    }

                    *state = PlatformState::MovingToEnd;
                    self.animation.time = 0.0;
                }
                PlatformState::StoppedEnd => {
                    if self.animation.time <= 2.0 {
                        return;
                    }

                    *state = PlatformState::MovingToStart;
                    self.animation.time = 0.0;
                }
                PlatformState::MovingToStart => {
                    if self.position <= self.start {
                        *state = PlatformState::StoppedStart;
                        self.animation.time = 0.0;
                        return;
                    }

                    new_position = self.position - {
                        Point {
                            x: self.animation.speed * delta,
                            y: self.animation.speed * delta,
                        } * direction.vector()
                    };

                    if new_position <= self.start {
                        new_position = self.start
                    }
                    self.position = new_position;
                }

                PlatformState::MovingToEnd => {
                    if self.position >= self.end {
                        *state = PlatformState::StoppedEnd;
                        self.animation.time = 0.0;
                        return;
                    }

                    new_position = self.position + {
                        Point {
                            x: self.animation.speed * delta,
                            y: self.animation.speed * delta,
                        } * direction.vector()
                    };
                    if new_position >= self.end {
                        new_position = self.end
                    }
                    self.position = new_position;
                }
            },
        }

        dbg!(self.position, &self.kind);
    }

    pub fn render(&self) {
        draw_rectangle(
            self.position.x - self.width / 2.0,
            self.position.y - self.height / 2.0,
            self.width,
            self.height,
            GREEN,
        );
    }
}
