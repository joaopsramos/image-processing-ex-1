mod bullet;
mod enemy;
mod enemy_spawner;
mod triangle;

use bullet::Bullet;
use enemy::Enemy;
use enemy_spawner::Spawner;
use macroquad::prelude::*;
use triangle::Triangle;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut triangle = create_triangle();
    let mut spawner = Spawner::new();

    let mut bullets = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();

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

        triangle.tick(mouse_position());
        triangle.draw();

        if let Some(new_enemy) = spawner.spawn() {
            enemies.push(new_enemy);
        }

        'outter: for b in bullets.iter_mut() {
            if b.hit {
                continue;
            }

            b.tick();

            for enemy in enemies.iter_mut() {
                if b.shape.overlaps(&enemy.shape) {
                    enemy.take_damage();
                    b.hit = true;
                    continue 'outter;
                }
            }

            b.draw();
        }

        bullets.retain(|b| {
            !b.hit
                || (b.shape.x > 0.0
                    && b.shape.x < screen_width()
                    && b.shape.y > 0.0
                    && b.shape.y < screen_height())
        });

        for enemy in enemies.iter_mut() {
            if enemy.heatlh > 0.0 {
                enemy.tick(&triangle);
                enemy.draw();
            }
        }

        next_frame().await
    }
}

fn create_triangle() -> Triangle {
    Triangle::new(25.0, BLUE)
}

fn handle_inputs(triangle: &mut Triangle, bullets: &mut Vec<Bullet>) {
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

    if is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::Space) {
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
