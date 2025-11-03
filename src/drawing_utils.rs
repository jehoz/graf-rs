use macroquad::{
    color::Color,
    math::{vec2, Vec2},
    shapes::{draw_line, draw_poly},
};

use crate::{devices::Device, session::DrawContext};

type Path = Vec<Vec2>;

/// Computes a path between two points using at-most three straight lines which
/// are horizontal, vertical, or at a 45-degree angle.
pub fn three_segment_path(from: Vec2, to: Vec2) -> Path {
    let delta = to - from;
    if delta.x.abs() > delta.y.abs() {
        let span = delta.x.signum() * (delta.x.abs() - delta.y.abs()) / 2.0;
        vec![
            from,
            vec2(from.x + span, from.y),
            vec2(to.x - span, to.y),
            to,
        ]
    } else {
        let span = delta.y.signum() * (delta.y.abs() - delta.x.abs()) / 2.0;
        vec![
            from,
            vec2(from.x, from.y + span),
            vec2(to.x, to.y - span),
            to,
        ]
    }
}

pub fn draw_line_path(path: Path, thickness: f32, color: Color) {
    for (from, to) in path.iter().zip(path.iter().skip(1)) {
        draw_line(from.x, from.y, to.x, to.y, thickness, color);
    }
}

pub fn draw_arrow_path(path: Path, thickness: f32, head_size: f32, color: Color) {
    draw_line_path(path.clone(), thickness, color);

    let p1 = path.last().unwrap();
    let p0 = path.iter().rev().skip(1).next().unwrap();
    let rotation = vec2(1.0, 0.0).angle_between(*p1 - *p0).to_degrees();

    let arrow_pos = *p1 - (*p1 - *p0).normalize() * head_size;
    draw_poly(arrow_pos.x, arrow_pos.y, 3, head_size, rotation, color);
}

pub fn draw_wire(from: Vec2, to: Vec2, color: Color) {
    let path = vec![from, to];
    draw_arrow_path(path, 1.0, 6.0, color);
}

pub fn draw_wire_from_device<D: Device + ?Sized>(draw_ctx: &DrawContext, from_dev: &D, to: Vec2, color: Color) {
    let from_pos = from_dev.closest_border_point(draw_ctx.viewport_to_world(to), 3.0);
    draw_wire(draw_ctx.world_to_viewport(from_pos), to, color);
}

pub fn draw_wire_between_devices<D: Device + ?Sized>(draw_ctx: &DrawContext, from_dev: &D, to_dev: &D, color: Color) {
    let from_pos = from_dev.closest_border_point(to_dev.get_position(), 3.0);
    let to_pos = to_dev.closest_border_point(from_dev.get_position(), 3.0);
    draw_wire(draw_ctx.world_to_viewport(from_pos), draw_ctx.world_to_viewport(to_pos), color);
}
