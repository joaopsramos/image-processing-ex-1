use std::time::Instant;

use macroquad::{
    prelude::{Color, Rect, Vec2, WHITE},
    shapes::draw_rectangle,
};

use crate::triangle::Triangle;

const HEALTH: f32 = 100.0;
const HEALTH_LOST_PER_HIT: f32 = 10.0;
const HIT_COLOR: Color = WHITE;
const HIT_DURATION_MS: u128 = 25;

pub struct Enemy {
    pub shape: Rect,
    color: Color,
    pub heatlh: f32,
    last_time_hit: Option<Instant>,
}

impl Enemy {
    pub fn new(pos: Vec2, width: f32, height: f32, color: Color) -> Self {
        Self {
            shape: Rect::new(pos.x, pos.y, width, height),
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
    }

    pub fn draw(&self) {
        let color = if self.last_time_hit.is_some() {
            HIT_COLOR
        } else {
            self.color
        };

        draw_rectangle(
            self.shape.x,
            self.shape.y,
            self.shape.w,
            self.shape.h,
            color,
        )
    }

    pub fn take_damage(&mut self) {
        self.heatlh -= HEALTH_LOST_PER_HIT;
        self.last_time_hit = Some(Instant::now());
    }
}
