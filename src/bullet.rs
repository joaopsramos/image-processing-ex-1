use macroquad::{
    prelude::{vec2, Vec2, RED},
    shapes::draw_line,
};

use crate::vertices_mut;

const BULLET_WIDTH: f32 = 2.0;
const BULLET_HEIGHT: f32 = 8.0;
const BULLET_SPEED: f32 = 4.2;

pub struct Bullet {
    pub v1: Vec2,
    pub v2: Vec2,
    pub v3: Vec2,
    pub v4: Vec2,
    speed: Vec2,
    pub hit: bool,
}

impl Bullet {
    pub fn new(pos: Vec2, angle: f32) -> Self {
        let pos = vec2(pos.x - BULLET_WIDTH / 2.0, pos.y);

        let v1 = pos;
        let v2 = vec2(pos.x + BULLET_WIDTH, pos.y);
        let v3 = vec2(pos.x + BULLET_WIDTH, pos.y + BULLET_HEIGHT);
        let v4 = vec2(pos.x, pos.y + BULLET_HEIGHT);
        let speed = vec2(angle.sin(), angle.cos()) * vec2(BULLET_SPEED, BULLET_SPEED);

        let mut bullet = Self {
            v1,
            v2,
            v3,
            v4,
            speed,
            hit: false,
        };

        bullet.rotate(-angle);

        bullet
    }

    pub fn tick(&mut self) {
        for v in vertices_mut!(self, v1, v2, v3, v4) {
            *v += self.speed;
        }
    }

    fn rotate(&mut self, angle: f32) {
        let center = self.center();
        let sin = angle.sin();
        let cos = angle.cos();

        for v in vertices_mut!(self, v1, v2, v3, v4) {
            let x = v.x - center.x;
            let y = v.y - center.y;

            let new_x = cos * x - sin * y;
            let new_y = sin * x + cos * y;

            *v = vec2(new_x + center.x, new_y + center.y) + self.speed;
        }
    }

    fn center(&self) -> Vec2 {
        (self.v1 + self.v2 + self.v3 + self.v4) / 4.0
    }

    pub fn draw(&self) {
        let Self { v1, v2, v3, v4, .. } = self;
        draw_line(v1.x, v1.y, v2.x, v2.y, 1.0, RED);
        draw_line(v2.x, v2.y, v3.x, v3.y, 1.0, RED);
        draw_line(v3.x, v3.y, v4.x, v4.y, 1.0, RED);
        draw_line(v4.x, v4.y, v1.x, v1.y, 1.0, RED);
    }
}
