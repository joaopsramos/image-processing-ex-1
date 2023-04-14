use macroquad::{
    prelude::{Color, Rect, Vec2},
    shapes::draw_rectangle,
};

const HEALTH: f32 = 100.0;

pub struct Enemy {
    pub shape: Rect,
    color: Color,
    pub heatlh: f32,
}

impl Enemy {
    pub fn new(pos: Vec2, width: f32, height: f32, color: Color) -> Self {
        Self {
            shape: Rect::new(pos.x, pos.y, width, height),
            color,
            heatlh: HEALTH,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.shape.x,
            self.shape.y,
            self.shape.w,
            self.shape.h,
            self.color,
        )
    }
}
