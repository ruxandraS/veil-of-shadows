use crate::collider::Collider;
use crate::light_shard::LightShard;
use crate::utility::Point;

use macroquad::prelude::*;

pub struct World {
    darkness: f32,
    light_shards: Vec<LightShard>,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            darkness: 20.0,
            light_shards: Vec::new(),
        };

        world.generate_light_shards();

        world
    }

    fn generate_light_shards(&mut self) {
        // Generate light shards
        let light_shard = LightShard::new(Point {
            x: 100.0,
            y: screen_height() - 40.0,
        });

        self.light_shards.push(light_shard);
    }

    pub fn check_collisions(&mut self, player_collider: &Collider) {
        for light_shard in &mut self.light_shards {
            if light_shard.is_collected {
                continue;
            }

            if player_collider.collides_with(&light_shard.collider) {
                light_shard.is_collected = true;
                self.darkness -= 5.0;

                println!("Darkness diminished: {}", self.darkness);
            }
        }
    }

    pub fn render(&self) {
        for light_shard in &self.light_shards {
            light_shard.render();
        }
    }
}
