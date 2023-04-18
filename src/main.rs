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
    let mut is_over = false;
    let mut score = 0;

    loop {
        clear_background(BLACK);

        draw_text(
            format!("fps = {}", get_fps()).as_str(),
            10.0,
            20.0,
            20.0,
            YELLOW,
        );

        draw_text(
            format!("score = {}", score).as_str(),
            screen_width() - 100.0,
            20.0,
            20.0,
            YELLOW,
        );

        if is_over {
            let game_over_label = "Game Over";
            let measure = measure_text(game_over_label, None, 100, 1.0);
            let button_x = (screen_width() - 200.0) / 2.0;
            let button_y = (screen_height() + 150.0) / 2.0;
            let button_width = 200.0;
            let button_height = 50.0;

            draw_text(
                game_over_label,
                (screen_width() - measure.width) / 2.0,
                (screen_height() - measure.height) / 2.0,
                100.0,
                YELLOW,
            );
            draw_rectangle(button_x, button_y, button_width, button_height, RED);
            draw_text("Try Again", button_x + 35.0, button_y + 30.0, 32.0, WHITE);
            if is_mouse_button_down(MouseButton::Left)
                && is_mouse_button_pressed(MouseButton::Left)
                && is_mouse_inside_button(button_x, button_y, button_width, button_height)
            {
                score = 0;
                is_over = false;
                enemies.clear();
                bullets.clear();
            }
            next_frame().await;
            continue;
        }

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
                    let died = enemy.take_damage();
                    if died {
                        score += 1;
                    }
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
            let hit_points = [v1, v2, v3, triangle.center()];

            if enemy.collide_with_hit_points(&hit_points) {
                is_over = true;
                // exit(0)
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

// Verifica se o cursor do mouse está dentro do retângulo do botão
fn is_mouse_inside_button(
    button_x: f32,
    button_y: f32,
    button_width: f32,
    button_height: f32,
) -> bool {
    let mouse_position = mouse_position();
    let mouse_x = mouse_position.0;
    let mouse_y = mouse_position.1;
    mouse_x >= button_x
        && mouse_x <= button_x + button_width
        && mouse_y >= button_y
        && mouse_y <= button_y + button_height
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
