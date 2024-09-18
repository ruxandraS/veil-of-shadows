use crate::{collider::Collider, light_shard::LightShard, platform::Platform, utility::Point};

use macroquad::prelude::*;

pub struct World {
    darkness: f32,
    light_shards: Vec<LightShard>,
    platforms: Vec<Platform>,
}

impl World {
    pub async fn new() -> Self {
        let mut world = Self {
            darkness: 20.0,
            light_shards: Vec::new(),
            platforms: Vec::new(),
        };

        world.generate_light_shards().await;
        world.generate_platforms();

        world
    }

    async fn generate_light_shards(&mut self) {
        // Generate light shards
        let light_shard = LightShard::new(Point {
            x: 100.0,
            y: screen_height() - 40.0,
        })
        .await;

        self.light_shards.push(light_shard);
    }

    fn generate_platforms(&mut self) {
        let elevator = Platform::elevator(
            Point {
                x: 200.0,
                y: screen_height() - 100.0,
            },
            40.0,
            10.0,
            Point { x: 0.0, y: 0.0 },
            Point { x: 0.0, y: 50.0 },
        );
        let raft = Platform::raft(
            Point {
                x: 200.0,
                y: screen_height() - 40.0,
            },
            40.0,
            10.0,
            Point { x: 0.0, y: 0.0 },
            Point { x: 50.0, y: 0.0 },
        );

        self.platforms.push(elevator);
        self.platforms.push(raft);
    }

    pub fn update(&mut self, delta: f32) {
        // Update light shards
        for light_shard in &mut self.light_shards {
            light_shard.update(delta);
        }

        for platform in &mut self.platforms {
            platform.update(delta);
        }
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

        for platform in &self.platforms {
            platform.render();
        }
    }
}
