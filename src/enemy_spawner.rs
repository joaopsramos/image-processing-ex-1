use std::time::Instant;

use crate::enemy::Enemy;

use macroquad::{
    prelude::{vec2, PURPLE},
    rand::RandomRange,
    window::{screen_height, screen_width},
};

const DEFAULT_COOLDOWN: f32 = 5.0;

pub struct Spawner {
    pub cooldown_time: f32,
    pub last_spawn_time: Instant,
}

impl Spawner {
    pub fn new() -> Self {
        Spawner {
            cooldown_time: DEFAULT_COOLDOWN,
            last_spawn_time: Instant::now(),
        }
    }

    pub fn spawn(&mut self) -> Option<Enemy> {
        if !self.can_spawn_enemy() {
            return None;
        }

        self.last_spawn_time = Instant::now();

        let random = RandomRange::gen_range(1.8, 2.2);
        let random_x = screen_width() / random;
        let random_y = screen_height() / random;

        Some(Enemy::new(vec2(random_x, random_y), PURPLE, 100.0))
    }

    fn can_spawn_enemy(&self) -> bool {
        let elapsed_time = self.last_spawn_time.elapsed().as_secs_f32();
        elapsed_time >= DEFAULT_COOLDOWN
    }
}
