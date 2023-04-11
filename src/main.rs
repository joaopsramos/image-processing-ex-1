mod circle_area;
mod triangle;

use circle_area::CircleArea;
use macroquad::prelude::*;
use triangle::Triangle;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut triangle = create_triangle();

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

        triangle.mov_area.draw();
        triangle.draw();

        next_frame().await
    }
}

fn create_triangle() -> Triangle {
    let middle_x = screen_width() / 2.0;
    let middle_y = screen_height() / 2.0;
    let circle_area = CircleArea::new(vec2(middle_x, middle_y), 200.0);

    Triangle::new(25.0, circle_area, BLUE)
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

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
