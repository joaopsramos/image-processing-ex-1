use std::time::Instant;

use macroquad::{
    prelude::{Circle, Color, Vec2, WHITE},
    shapes::draw_circle,
};

use crate::triangle::Triangle;

const HEALTH: f32 = 100.0;
const HEALTH_LOST_PER_HIT: f32 = 10.0;
const HIT_COLOR: Color = WHITE;
const HIT_DURATION_MS: u128 = 25;
const DEFAULT_INTERPOLATION: f32 = 0.002;

pub struct Enemy {
    pub shape: Circle,
    color: Color,
    pub heatlh: f32,
    last_time_hit: Option<Instant>,
}

impl Enemy {
    pub fn new(pos: Vec2, color: Color, r: f32) -> Self {
        Self {
            shape: Circle::new(pos.x, pos.y, r),
            color,
            heatlh: HEALTH,
            last_time_hit: None,
        }
    }

    pub fn tick(&mut self, player: &Triangle) {
        if let Some(time) = self.last_time_hit {
            if time.elapsed().as_millis() >= HIT_DURATION_MS {
                self.last_time_hit = None;
            }
        }

        let start_pos = Vec2::new(self.shape.x, self.shape.y);
        let end_pos = player.center();

        let new_pos = Vec2::lerp(start_pos, end_pos, DEFAULT_INTERPOLATION);

        self.shape.move_to(new_pos);
    }

    pub fn draw(&self) {
        let color = if self.last_time_hit.is_some() {
            HIT_COLOR
        } else {
            self.color
        };

        draw_circle(self.shape.x, self.shape.y, self.shape.r, color)
    }

    pub fn take_damage(&mut self) {
        self.heatlh -= HEALTH_LOST_PER_HIT;
        self.last_time_hit = Some(Instant::now());

        self.shape.r = self.heatlh;
    }
}
