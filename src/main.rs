mod triangle;

use macroquad::prelude::*;
use triangle::Triangle;

#[macroquad::main("BasicShapes")]
async fn main() {
    let size = 25.0;
    let x = screen_width() / 2.0;
    let y = screen_height() / 2.0;

    let v1 = vec2(x, y + 50.0);
    let v2 = vec2(x - size, y - size);
    let v3 = vec2(x + size, y - size);

    let mut triangle = Triangle::new(v1, v2, v3, BLUE);

    loop {
        clear_background(BLACK);

        draw_text(
            format!("fps = {}", get_fps()).as_str(),
            10.0,
            20.0,
            20.0,
            YELLOW,
        );

        handle_inputs(&mut triangle);

        triangle.draw();

        next_frame().await
    }
}

fn handle_inputs(triangle: &mut Triangle) {
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
}

trait Object {
    fn draw(&self);
    fn rotate(&mut self, degrees: f32);
    fn translate(&mut self, direction: Direction);
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

