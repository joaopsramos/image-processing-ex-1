use macroquad::{
    prelude::{vec2, Rect, Vec2, RED},
    shapes::draw_rectangle,
};

const BULLET_WIDTH: f32 = 6.0;
const BULLET_HEIGHT: f32 = 6.0;
const BULLET_SPEED: f32 = 4.2;

pub struct Bullet {
    pub shape: Rect,
    speed: Vec2,
    pub hit: bool,
}

impl Bullet {
    pub fn new(pos: Vec2, speed: Vec2) -> Self {
        Self {
            shape: Rect::new(pos.x, pos.y, BULLET_WIDTH, BULLET_HEIGHT),
            speed: speed * vec2(BULLET_SPEED, BULLET_SPEED),
            hit: false,
        }
    }

    pub fn tick(&mut self) {
        self.shape.x += self.speed.x;
        self.shape.y += self.speed.y;
    }

    pub fn draw(&self) {
        draw_rectangle(self.shape.x, self.shape.y, self.shape.w, self.shape.h, RED)
    }
}
