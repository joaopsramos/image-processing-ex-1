mod bullet;
mod circle_area;
mod rect;
mod triangle;

use bullet::Bullet;
use circle_area::CircleArea;
use macroquad::prelude::*;
use rect::Enemy;
use triangle::Triangle;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut triangle = create_triangle();
    let mut bullets = Vec::new();
    let mut rect = Enemy::new(
        vec2(screen_width() / 2.0, screen_height() / 2.0),
        20.0,
        20.0,
        PURPLE,
    );

    loop {
        clear_background(BLACK);

        draw_text(
            format!("fps = {}", get_fps()).as_str(),
            10.0,
            20.0,
            20.0,
            YELLOW,
        );

        handle_inputs(&mut triangle, &mut bullets);

        triangle.mov_area.draw();
        triangle.draw();

        for b in bullets.iter_mut() {
            b.tick();
            b.draw();

            if b.shape.overlaps_rect(&rect.shape) {
                rect.heatlh -= 1.0;
            }
        }

        bullets.retain(|b| {
            b.shape.x > 0.0
                && b.shape.x < screen_width()
                && b.shape.y > 0.0
                && b.shape.y < screen_height()
        });

        if rect.heatlh > 0.0 {
            rect.draw()
        }

        next_frame().await
    }
}

fn create_triangle() -> Triangle {
    let middle_x = screen_width() / 2.0;
    let middle_y = screen_height() / 2.0;
    let circle_area = CircleArea::new(vec2(middle_x, middle_y), 200.0);

    Triangle::new(25.0, circle_area, BLUE)
}

fn handle_inputs(triangle: &mut Triangle, bullets: &mut Vec<Bullet>) {
    if is_key_down(KeyCode::E) {
        triangle.rotate(5.0);
    }

    if is_key_down(KeyCode::Q) {
        triangle.rotate(-5.0);
    }

    if is_key_down(KeyCode::W) {
        triangle.translate(Direction::Up);
    }

    if is_key_down(KeyCode::A) {
        triangle.translate(Direction::Left);
    }

    if is_key_down(KeyCode::S) {
        triangle.translate(Direction::Down);
    }

    if is_key_down(KeyCode::D) {
        triangle.translate(Direction::Right);
    }

    if is_key_down(KeyCode::Space) {
        if let Some(new_bullet) = triangle.spawn_bullet() {
            bullets.push(new_bullet);
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
