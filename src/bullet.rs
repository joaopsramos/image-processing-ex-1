use macroquad::{
    prelude::{vec2, Circle, Vec2, RED},
    shapes::draw_circle,
};

const BULLET_RAIUS: f32 = 4.0;
const BULLET_SPEED: f32 = 2.2;

pub struct Bullet {
    pub shape: Circle,
    speed: Vec2,
    pub hit: bool,
}

impl Bullet {
    pub fn new(pos: Vec2, speed: Vec2) -> Self {
        Self {
            shape: Circle::new(pos.x, pos.y, BULLET_RAIUS),
            speed: speed * vec2(BULLET_SPEED, BULLET_SPEED),
            hit: false,
        }
    }

    pub fn tick(&mut self) {
        self.shape.x += self.speed.x;
        self.shape.y += self.speed.y;
    }

    pub fn draw(&self) {
        draw_circle(self.shape.x, self.shape.y, self.shape.r, RED)
    }
}
