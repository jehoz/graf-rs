use macroquad::{
    color::Color,
    math::{vec2, Vec2},
    shapes::{draw_line, draw_poly},
};

pub fn draw_arrow(from: Vec2, to: Vec2, thickness: f32, head_size: f32, color: Color) {
    draw_line(from.x, from.y, to.x, to.y, thickness, color);

    let rotation = vec2(1.0, 0.0).angle_between(to - from).to_degrees();
    draw_poly(to.x, to.y, 3, head_size, rotation, color);
}
