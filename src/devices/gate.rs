use macroquad::{
    color::{BLACK, WHITE},
    math::{vec2, Vec2},
    shapes::{draw_circle, draw_line, draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};

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

    fn update(&mut self, inputs: Vec<bool>) -> Option<bool> {
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

    fn draw(&self) {
        let Vec2 { x, y } = self.position;
        draw_rectangle(
            x - GATE_WIDTH / 2.,
            y - GATE_WIDTH / 2.,
            GATE_WIDTH,
            GATE_WIDTH,
            BLACK,
        );
        draw_rectangle_lines(
            x - GATE_WIDTH / 2.,
            y - GATE_WIDTH / 2.,
            GATE_WIDTH,
            GATE_WIDTH,
            2.0,
            WHITE,
        );

        draw_AND(x, y, GATE_WIDTH * 0.5);
    }

    fn inspector(&mut self, ui: &mut macroquad::ui::Ui) {
        todo!()
    }

    fn input_arity(&self) -> Arity {
        Arity::NAry
    }

    fn has_output(&self) -> bool {
        true
    }
}

fn draw_AND(x: f32, y: f32, scale: f32) {
    let top = y - scale / 2.0;
    let bottom = y + scale / 2.0;
    let left = x - scale / 2.0;
    let right = x + scale / 2.0;

    draw_line(left, bottom, x, top, 1.0, WHITE);
    draw_line(x, top, right, bottom, 1.0, WHITE);

    draw_circle(left, bottom, 2.0, WHITE);
    draw_circle(right, bottom, 2.0, WHITE);
    draw_circle(x, top, 2.0, WHITE);
}
