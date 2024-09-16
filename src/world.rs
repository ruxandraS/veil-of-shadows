use crate::light_shard;
use crate::utility;

use macroquad::prelude::*;

pub struct World {
    darkness: u8,
    light_shards: Vec<light_shard::LightShard>,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            darkness: 50,
            light_shards: Vec::new(),
        };

        world.generate_light_shards();

        world
    }

    fn generate_light_shards(&mut self) {
        // Generate light shards
        let light_shard = light_shard::LightShard::new(utility::Point {
            x: 100.0,
            y: screen_height() - 40.0,
        });

        self.light_shards.push(light_shard);
    }

    pub fn render(&self) {
        for light_shard in &self.light_shards {
            light_shard.render();
        }
    }
}
