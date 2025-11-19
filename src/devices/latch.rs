use macroquad::{
    math::Vec2,
    shapes::{draw_poly, draw_poly_lines}
};

use egui::{FontId, RichText};

use crate::session::{DrawContext, UpdateContext};
use crate::devices::{Arity, Device, LATCH_RADIUS};

#[derive(Clone)]
pub struct Latch {
    position: Vec2,

    is_on: bool,
    prev_input: bool

}

impl Latch {
    pub fn new(position: Vec2) -> Self {
        Latch {
            position,

            is_on: false,
            prev_input: false,
        }
    }
}

impl Device for Latch {
    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    fn closest_border_point(&self, point: Vec2, padding: f32) -> Vec2 {
        let delta = point - self.position;
        self.position + delta.normalize() * (LATCH_RADIUS + padding)
    }

    fn is_point_inside(&self, pt: Vec2) -> bool {
        self.position.distance(pt) <= LATCH_RADIUS
    }

    fn update(&mut self, _ctx: &mut UpdateContext, inputs: Vec<bool>) -> Option<bool> {
        let input_on = inputs.first().map(|x| *x).unwrap_or(false);

        if input_on && !self.prev_input {
            self.is_on = !self.is_on;
        }
        self.prev_input = input_on;

        Some(self.is_on)
    }

    fn draw(&self, ctx: &DrawContext, is_selected: bool) {
        let Vec2 { x, y } = ctx.world_to_viewport(self.position);

        let angle = 90.0;

        if is_selected {
            draw_poly_lines(x, y, 3, LATCH_RADIUS + 4.0, angle, 2.0, ctx.fg_color.with_alpha(0.5));
        }

        draw_poly_lines(x, y, 3, LATCH_RADIUS, angle, 2.0, ctx.fg_color);
        draw_poly(x, y, 3, LATCH_RADIUS, angle, ctx.bg_color);
        
        if self.is_on {
            draw_poly(x, y, 3, LATCH_RADIUS / 2.0, angle, ctx.fg_color);
        }
    }

    fn reset(&mut self) {
        self.is_on = false;
    }

    fn inspector(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("Latch")
                .font(FontId::proportional(16.0))
                .strong(),
        );
        ui.separator();

        ui.checkbox(&mut self.is_on, "On");
    }

    fn input_arity(&self) -> super::Arity {
        Arity::Unary
    }

    fn has_output(&self) -> bool {
        true
    }

    fn clone_dyn(&self) -> Box<dyn Device> {
        Box::new(self.clone())
    }
}
