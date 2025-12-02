use egui::Color32;
use macroquad::{
    color::Color,
    math::{vec2, Vec2},
    shapes::{draw_line, draw_poly},
};

use crate::{app::DrawContext, devices::Device};

pub struct ColorPalette {
    pub fg_0: Color,
    pub fg_1: Color,
    pub fg_2: Color,
    pub fg_3: Color,
    pub bg_0: Color,
    pub bg_1: Color,
    pub bg_2: Color,
    pub bg_3: Color,
    pub error: Color,
}

pub fn color_to_color32(c: Color) -> Color32 {
    let [r, g, b, _a] = c.into();
    Color32::from_rgb(r, g, b)
}

pub fn draw_line_v(from: Vec2, to: Vec2, thickness: f32, color: Color) {
    draw_line(from.x, from.y, to.x, to.y, thickness, color)
}

pub fn draw_dashed_line(from: Vec2, to: Vec2, color: Color, dash_size: f32) {
    let length = (to - from).length();
    let num_segments = (length / dash_size).ceil() as i32;
    let dx = (to - from).x / num_segments as f32;
    let dy = (to - from).y / num_segments as f32;
    let mut prev_x = from.x;
    let mut prev_y = from.y;
    for _ in 0..num_segments {
        let x = prev_x + dx / 2.0;
        let y = prev_y + dy / 2.0;
        draw_line(prev_x, prev_y, x, y, 1.0, color);
        prev_x += dx;
        prev_y += dy;
    }
}

pub fn draw_arrow(from: Vec2, to: Vec2, thickness: f32, head_size: f32, color: Color) {
    let rotation = vec2(1.0, 0.0).angle_between(to - from).to_degrees();
    let arrow_pos = to - (to - from).normalize() * head_size;

    draw_line_v(from, arrow_pos, thickness, color);
    draw_poly(arrow_pos.x, arrow_pos.y, 3, head_size, rotation, color);

    draw_line_v(from, arrow_pos, 1.5, macroquad::color::BLACK);
    draw_poly(
        arrow_pos.x,
        arrow_pos.y,
        3,
        head_size - 1.5,
        rotation,
        macroquad::color::BLACK,
    );
}

pub fn draw_wire(from: Vec2, to: Vec2, color: Color) {
    draw_arrow(from, to, 3.0, 6.0, color);
}

pub fn draw_wire_from_device<D: Device + ?Sized>(
    draw_ctx: &DrawContext,
    from_dev: &D,
    to: Vec2,
    color: Color,
) {
    let from_pos = from_dev.closest_border_point(draw_ctx.viewport_to_world(to), 3.0);
    draw_wire(draw_ctx.world_to_viewport(from_pos), to, color);
}

pub fn draw_wire_between_devices<D: Device + ?Sized>(
    draw_ctx: &DrawContext,
    from_dev: &D,
    to_dev: &D,
    color: Color,
) {
    let from_pos = from_dev.closest_border_point(to_dev.get_position(), 3.0);
    let to_pos = to_dev.closest_border_point(from_dev.get_position(), 3.0);
    draw_wire(
        draw_ctx.world_to_viewport(from_pos),
        draw_ctx.world_to_viewport(to_pos),
        color,
    );
}
