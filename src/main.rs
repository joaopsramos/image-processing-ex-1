mod bullet;
mod enemy;
mod enemy_spawner;
mod triangle;

use bullet::Bullet;
use enemy::Enemy;
use enemy_spawner::Spawner;
use macroquad::prelude::*;
use std::process::exit;
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

        'outer: for b in bullets.iter_mut() {
            if b.hit {
                continue;
            }

            b.tick();

            let Bullet { v1, v2, v3, v4, .. } = b;
            let hit_points = [*v1, *v2, *v3, *v4];

            for enemy in enemies.iter_mut() {
                if enemy.collide_with_hit_points(&hit_points) {
                    enemy.take_damage();
                    b.hit = true;
                    continue 'outer;
                }
            }

            b.draw();
        }

        for enemy in enemies.iter_mut() {
            if enemy.heatlh > 0.0 {
                enemy.tick(&triangle);
                enemy.draw();
            }

            let Triangle { v1, v2, v3, .. } = triangle;
            let hit_points = [v1, v2, v3];

            if enemy.collide_with_hit_points(&hit_points) {
                exit(0)
            }
        }

        bullets.retain(|b| {
            !b.hit
                && (b.v1.x > 0.0
                    && b.v1.x < screen_width()
                    && b.v1.y > 0.0
                    && b.v1.y < screen_height())
        });

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
