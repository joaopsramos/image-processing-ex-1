use macroquad::{
    prelude::{vec2, Color, Vec2},
    shapes::draw_triangle,
    window::{screen_height, screen_width},
};

use crate::{Direction, Object};

pub struct Triangle {
    pub v1: Vec2,
    pub v2: Vec2,
    pub v3: Vec2,
    pub color: Color,
}

impl Triangle {
    pub fn new(v1: Vec2, v2: Vec2, v3: Vec2, color: Color) -> Self {
        Triangle { v1, v2, v3, color }
    }

    fn get_center(&self) -> Vec2 {
        let Triangle { v1, v2, v3, .. } = self;

        let x_center = (v1.x + v2.x + v3.x) / 3.0;
        let y_center = (v1.y + v2.y + v3.y) / 3.0;

        vec2(x_center, y_center)
    }

    pub fn get_pointing_angle(&self) -> f32 {
        let Triangle { v1, .. } = self;
        let center = self.get_center();

        let dx = v1.x - center.x;
        let dy = v1.y - center.y;

        dx.atan2(dy)
    }
}

#[macro_export]
macro_rules! vertices_mut {
    ($self:ident, $($field:ident),+) => {
        [$(&mut $self.$field),+]
    };
}

impl Object for Triangle {
    fn draw(&self) {
        draw_triangle(self.v1, self.v2, self.v3, self.color);
    }

    fn rotate(&mut self, degrees: f32) {
        let angle = degrees.to_radians();
        let center = self.get_center();

        for v in vertices_mut!(self, v1, v2, v3) {
            let x = v.x - center.x;
            let y = v.y - center.y;

            let new_x = angle.cos() * x - angle.sin() * y;
            let new_y = angle.sin() * x + angle.cos() * y;

            *v = vec2(new_x + center.x, new_y + center.y)
        }
    }

    fn translate(&mut self, direction: Direction) {
        let pixels = 10.0;

        for p in vertices_mut!(self, v1, v2, v3) {
            *p = match direction {
                Direction::Up => vec2(p.x, p.y - pixels),
                Direction::Down => vec2(p.x, p.y + pixels),
                Direction::Left => vec2(p.x - pixels, p.y),
                Direction::Right => vec2(p.x + pixels, p.y),
            }
        }
    }
}
