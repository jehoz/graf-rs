use macroquad::{color::WHITE, math::Vec2, shapes::draw_circle};

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

    fn is_point_inside(&self, pt: Vec2) -> bool {
        self.position.distance(pt) <= NOTE_RADIUS
    }

    fn draw(&self) {
        let Vec2 { x, y } = self.position;
        draw_circle(x, y, NOTE_RADIUS, WHITE);
    }

    fn input_arity(&self) -> Arity {
        Arity::Unary
    }

    fn has_output(&self) -> bool {
        false
    }
}
