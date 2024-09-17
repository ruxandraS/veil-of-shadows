pub struct Animation {
    pub time: f32,
    pub frame_size: u8,
    pub current_frame: u8,
    pub frame_count: u8,
    pub flip: bool,
}

impl Animation {
    pub fn new(frame_size: u8, frame_count: u8) -> Animation {
        Animation {
            time: 0.0,
            frame_size,
            current_frame: 0,
            frame_count,
            flip: false,
        }
    }

    pub fn reset_frame(&mut self) {
        self.current_frame = 0;
        self.time = 0.0;
    }

    pub fn next_frame(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.frame_count;
        self.time = 0.0;
    }
}