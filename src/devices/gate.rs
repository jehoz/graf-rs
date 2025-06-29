use macroquad::{
    math::Vec2,
    shapes::{draw_circle, draw_circle_lines, draw_line, draw_rectangle, draw_rectangle_lines},
};

use crate::session::{DrawContext, UpdateContext};

use super::{Arity, Device, GATE_WIDTH};

pub enum BooleanOperation {
    AND,
    OR,
    XOR,
    NAND,
    NOR,
    XNOR,
}

pub struct Gate {
    position: Vec2,
    operation: BooleanOperation,
}

impl Gate {
    pub fn new(position: Vec2) -> Self {
        Gate {
            position,
            operation: BooleanOperation::AND,
        }
    }
}

impl Device for Gate {
    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    fn closest_border_point(&self, point: Vec2, padding: f32) -> Vec2 {
        let padded_width = (GATE_WIDTH / 2.0) + padding;
        let u = f32::max(
            (point.x - self.position.x).abs(),
            (point.y - self.position.y).abs(),
        );

        padded_width * (point - self.position) / u + self.position
    }

    fn is_point_inside(&self, pt: Vec2) -> bool {
        let dx = (pt.x - self.position.x).abs();
        let dy = (pt.y - self.position.y).abs();
        dx <= GATE_WIDTH && dy <= GATE_WIDTH
    }

    fn update(&mut self, _ctx: &UpdateContext, inputs: Vec<bool>) -> Option<bool> {
        let out = match self.operation {
            BooleanOperation::AND => inputs.iter().fold(true, |acc, x| acc && *x),
            BooleanOperation::OR => inputs.iter().fold(false, |acc, x| acc || *x),
            BooleanOperation::XOR => inputs.iter().fold(false, |acc, x| acc != *x),
            BooleanOperation::NAND => !inputs.iter().fold(true, |acc, x| acc && *x),
            BooleanOperation::NOR => !inputs.iter().fold(false, |acc, x| acc || *x),
            BooleanOperation::XNOR => inputs.iter().fold(false, |acc, x| acc == *x),
        };
        Some(out)
    }

    fn draw(&self, ctx: &DrawContext) {
        let Vec2 { x, y } = self.position;
        draw_rectangle(
            x - GATE_WIDTH / 2.,
            y - GATE_WIDTH / 2.,
            GATE_WIDTH,
            GATE_WIDTH,
            ctx.bg_color,
        );
        draw_rectangle_lines(
            x - GATE_WIDTH / 2.,
            y - GATE_WIDTH / 2.,
            GATE_WIDTH,
            GATE_WIDTH,
            2.0,
            ctx.fg_color,
        );

        draw_symbol(ctx, x, y, GATE_WIDTH * 0.5, &self.operation);
    }

    fn inspector(&mut self, ui: &mut macroquad::ui::Ui) {
        ui.label(None, "Edit Gate");
        if ui.button(None, "AND") {
            self.operation = BooleanOperation::AND;
        }
        if ui.button(None, "OR") {
            self.operation = BooleanOperation::OR;
        }
        if ui.button(None, "XOR") {
            self.operation = BooleanOperation::XOR;
        }
        if ui.button(None, "NAND") {
            self.operation = BooleanOperation::NAND;
        }
        if ui.button(None, "NOR") {
            self.operation = BooleanOperation::NOR;
        }
        if ui.button(None, "XNOR") {
            self.operation = BooleanOperation::XNOR;
        }
    }

    fn input_arity(&self) -> Arity {
        Arity::NAry
    }

    fn has_output(&self) -> bool {
        true
    }
}

fn draw_symbol(ctx: &DrawContext, x: f32, y: f32, scale: f32, op: &BooleanOperation) {
    let top = y - scale / 2.0;
    let bottom = y + scale / 2.0;
    let left = x - scale / 2.0;
    let right = x + scale / 2.0;

    match op {
        BooleanOperation::AND => {
            draw_line(left, bottom, x, top, 1.0, ctx.fg_color);
            draw_line(x, top, right, bottom, 1.0, ctx.fg_color);

            draw_circle(left, bottom, 2.0, ctx.fg_color);
            draw_circle(right, bottom, 2.0, ctx.fg_color);
            draw_circle(x, top, 2.0, ctx.fg_color);
        }
        BooleanOperation::OR => {
            draw_line(left, top, x, bottom, 1.0, ctx.fg_color);
            draw_line(x, bottom, right, top, 1.0, ctx.fg_color);

            draw_circle(left, top, 2.0, ctx.fg_color);
            draw_circle(right, top, 2.0, ctx.fg_color);
            draw_circle(x, bottom, 2.0, ctx.fg_color);
        }
        BooleanOperation::XOR => {
            draw_circle_lines(x, y, scale / 2.0, 1.0, ctx.fg_color);
            draw_line(x, top, x, bottom, 1.0, ctx.fg_color);
            draw_line(left, y, right, y, 1.0, ctx.fg_color);

            draw_circle(x, top, 2.0, ctx.fg_color);
            draw_circle(x, bottom, 2.0, ctx.fg_color);
            draw_circle(left, y, 2.0, ctx.fg_color);
            draw_circle(right, y, 2.0, ctx.fg_color);
        }
        BooleanOperation::NAND => {
            draw_line(left, bottom, x, y, 1.0, ctx.fg_color);
            draw_line(x, y, right, bottom, 1.0, ctx.fg_color);
            draw_line(left, top, right, top, 1.0, ctx.fg_color);

            draw_circle(left, bottom, 2.0, ctx.fg_color);
            draw_circle(right, bottom, 2.0, ctx.fg_color);
            draw_circle(x, y, 2.0, ctx.fg_color);
            draw_circle(left, top, 2.0, ctx.fg_color);
            draw_circle(right, top, 2.0, ctx.fg_color);
        }
        BooleanOperation::NOR => {
            draw_line(left, y, x, bottom, 1.0, ctx.fg_color);
            draw_line(x, bottom, right, y, 1.0, ctx.fg_color);
            draw_line(left, top, right, top, 1.0, ctx.fg_color);

            draw_circle(left, y, 2.0, ctx.fg_color);
            draw_circle(right, y, 2.0, ctx.fg_color);
            draw_circle(x, bottom, 2.0, ctx.fg_color);
            draw_circle(left, top, 2.0, ctx.fg_color);
            draw_circle(right, top, 2.0, ctx.fg_color);
        }
        BooleanOperation::XNOR => {
            draw_line(left, top, right, top, 1.0, ctx.fg_color);
            draw_line(left, y, right, y, 1.0, ctx.fg_color);
            draw_line(left, bottom, right, bottom, 1.0, ctx.fg_color);

            draw_circle(left, top, 2.0, ctx.fg_color);
            draw_circle(right, top, 2.0, ctx.fg_color);
            draw_circle(left, y, 2.0, ctx.fg_color);
            draw_circle(right, y, 2.0, ctx.fg_color);
            draw_circle(left, bottom, 2.0, ctx.fg_color);
            draw_circle(right, bottom, 2.0, ctx.fg_color);
        }
    }
}
