use macroquad::{
    prelude::{Vec2, GREEN},
    shapes::draw_circle_lines,
};

#[derive(Debug)]
pub struct CircleArea {
    pub pos: Vec2,
    pub radius: f32,
}

impl CircleArea {
    pub fn new(pos: Vec2, radius: f32) -> Self {
        Self { pos, radius }
    }

    pub fn draw(&self) {
        draw_circle_lines(self.pos.x, self.pos.y, self.radius, 1.0, GREEN)
    }
}
