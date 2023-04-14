use std::time::Instant;

use macroquad::{
    prelude::{vec2, Color, Vec2},
    shapes::draw_triangle,
    time::get_time,
};

use crate::{bullet::Bullet, circle_area::CircleArea, Direction};

const BULLET_THROTTLE_DURATION_MS: f64 = 200.0;

pub struct Triangle {
    pub v1: Vec2,
    pub v2: Vec2,
    pub v3: Vec2,
    pub mov_area: CircleArea,
    pub color: Color,
    pub bullets: Vec<Bullet>,
    pub last_bullet_spawn: Instant,
}

#[macro_export]
macro_rules! vertices_mut {
    ($self:ident, $($field:ident),+) => {
        [$(&mut $self.$field),+]
    };
}

impl Triangle {
    pub fn new(size: f32, mov_area: CircleArea, color: Color) -> Self {
        let initial_angle = (90.0_f32 + 180.0).to_radians();

        let x = mov_area.pos.x + mov_area.radius * initial_angle.cos();
        let y = mov_area.pos.y + mov_area.radius * initial_angle.sin();

        Triangle {
            v1: vec2(x, y + size * 2.0),
            v2: vec2(x - size, y - size),
            v3: vec2(x + size, y - size),
            mov_area,
            color,
            bullets: Vec::new(),
            last_bullet_spawn: Instant::now(),
        }
    }

    fn center(&self) -> Vec2 {
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
        let angle = degrees.to_radians();
        let relative_to = self.mov_area.pos;

        for v in vertices_mut!(self, v1, v2, v3) {
            let x = v.x - relative_to.x;
            let y = v.y - relative_to.y;

            let new_x = angle.cos() * x - angle.sin() * y;
            let new_y = angle.sin() * x + angle.cos() * y;

            *v = vec2(new_x + relative_to.x, new_y + relative_to.y)
        }
    }

    pub fn translate(&mut self, direction: Direction) {
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

    pub fn spawn_bullet(&mut self) -> Option<Bullet> {
        if !self.can_spawn_bullet() {
            return None;
        }

        self.last_bullet_spawn = Instant::now();
        let angle = self.pointing_angle();

        Some(Bullet::new(self.v1, vec2(angle.sin(), angle.cos())))
    }

    fn can_spawn_bullet(&self) -> bool {
        let elapsed_time = self.last_bullet_spawn.elapsed().as_secs_f64() * 1000.0;

        elapsed_time >= BULLET_THROTTLE_DURATION_MS
    }
}
