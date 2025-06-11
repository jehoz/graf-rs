use macroquad::{
    color::WHITE,
    math::Vec2,
    shapes::{draw_circle, draw_circle_lines},
};

use super::{Arity, Device, NOTE_RADIUS};

pub struct Note {
    position: Vec2,

    midi_note: u8,
    velocity: u8,
    is_on: bool,
}

impl Note {
    pub fn new(position: Vec2) -> Self {
        Note {
            position,
            midi_note: 60,
            velocity: 100,
            is_on: false,
        }
    }
}

impl Device for Note {
    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    fn closest_border_point(&self, point: Vec2, padding: f32) -> Vec2 {
        let delta = point - self.position;
        self.position + delta.normalize() * (NOTE_RADIUS + padding)
    }

    fn is_point_inside(&self, pt: Vec2) -> bool {
        self.position.distance(pt) <= NOTE_RADIUS
    }

    fn update(&mut self, inputs: Vec<bool>) -> Option<bool> {
        if let Some(input_on) = inputs.first() {
            if *input_on && !self.is_on {
                // TODO send MIDI message
                self.is_on = true;
            } else if !(*input_on) && self.is_on {
                // TODO send MIDI message
                self.is_on = false;
            }
        }
        None
    }

    fn draw(&self) {
        let Vec2 { x, y } = self.position;
        draw_circle_lines(x, y, NOTE_RADIUS, 1.0, WHITE);
        if self.is_on {
            draw_circle(x, y, NOTE_RADIUS / 2.0, WHITE);
        }
    }

    fn inspector(&mut self, ui: &mut macroquad::ui::Ui) {
        todo!()
    }

    fn input_arity(&self) -> Arity {
        Arity::Unary
    }

    fn has_output(&self) -> bool {
        false
    }
}
