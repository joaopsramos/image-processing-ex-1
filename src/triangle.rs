use std::time::Instant;

use macroquad::{
    prelude::{vec2, Color, Vec2},
    shapes::draw_triangle,
    window::{screen_height, screen_width},
};

use crate::{bullet::Bullet, Direction};

const BULLET_THROTTLE_DURATION_MS: u128 = 100;

pub struct Triangle {
    pub v1: Vec2,
    pub v2: Vec2,
    pub v3: Vec2,
    pub color: Color,
    pub last_bullet_spawn: Instant,
}

#[macro_export]
macro_rules! vertices_mut {
    ($self:ident, $($field:ident),+) => {
        [$(&mut $self.$field),+]
    };
}

impl Triangle {
    pub fn new(size: f32, color: Color) -> Self {
        let initial_angle = (90.0_f32 + 180.0).to_radians();

        let middle_x = screen_width() / 2.0;
        let middle_y = screen_height() / 2.0;

        let x = middle_x + 200.0 * initial_angle.cos();
        let y = middle_y + 200.0 * initial_angle.sin();

        Triangle {
            v1: vec2(x, y + size * 2.0),
            v2: vec2(x - size, y - size),
            v3: vec2(x + size, y - size),
            color,
            last_bullet_spawn: Instant::now(),
        }
    }

    pub fn tick(&mut self, mouse_position: (f32, f32)) {
        let center = self.center();
        let angle = (mouse_position.1 - center.y).atan2(mouse_position.0 - center.x)
            - (self.v1.y - center.y).atan2(self.v1.x - center.x);
        self.rotate(angle);
    }

    pub fn center(&self) -> Vec2 {
        let Triangle { v1, v2, v3, .. } = self;

        let x_center = (v1.x + v2.x + v3.x) / 3.0;
        let y_center = (v1.y + v2.y + v3.y) / 3.0;

        vec2(x_center, y_center)
    }

    pub fn pointing_angle(&self) -> f32 {
        let Triangle { v1, .. } = self;
        let center = self.center();

        let dx = v1.x - center.x;
        let dy = v1.y - center.y;

        dx.atan2(dy)
    }

    pub fn draw(&self) {
        draw_triangle(self.v1, self.v2, self.v3, self.color);
    }

    pub fn rotate(&mut self, degrees: f32) {
        let relative_to = self.center();
        let cos = degrees.cos();
        let sin = degrees.sin();

        for v in vertices_mut!(self, v1, v2, v3) {
            let x = v.x - relative_to.x;
            let y = v.y - relative_to.y;

            let new_x = cos * x - sin * y;
            let new_y = sin * x + cos * y;

            *v = vec2(new_x + relative_to.x, new_y + relative_to.y)
        }
    }

    pub fn translate(&mut self, direction: Direction) {
        let pixels = 1.2;

        for p in vertices_mut!(self, v1, v2, v3) {
            *p = match direction {
                Direction::Up => vec2(p.x, p.y - pixels),
                Direction::Down => vec2(p.x, p.y + pixels),
                Direction::Left => vec2(p.x - pixels, p.y),
                Direction::Right => vec2(p.x + pixels, p.y),
            }
        }
    }

    pub fn spawn_bullet(&mut self) -> Option<Bullet> {
        if !self.can_spawn_bullet() {
            return None;
        }

        self.last_bullet_spawn = Instant::now();
        let angle = self.pointing_angle();

        Some(Bullet::new(self.v1, vec2(angle.sin(), angle.cos())))
    }

    fn can_spawn_bullet(&self) -> bool {
        let elapsed_time = self.last_bullet_spawn.elapsed().as_millis();

        elapsed_time >= BULLET_THROTTLE_DURATION_MS
    }
}
