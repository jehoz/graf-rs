use macroquad::{color::WHITE, math::Vec2, shapes::draw_circle_lines};

use super::{Arity, Device, CLOCK_RADIUS};

pub enum Frequency {
    Milliseconds(f32),
    Beats(BeatFraction),
}

pub struct BeatFraction {
    numerator: u16,
    denominator: u16,
}

pub struct Clock {
    position: Vec2,

    frequency: Frequency,
    duty_cycle: f32,
    offset: f32,
}

impl Clock {
    pub fn new(position: Vec2) -> Self {
        Clock {
            position,
            frequency: Frequency::Beats(BeatFraction {
                numerator: 1,
                denominator: 4,
            }),
            duty_cycle: 0.5,
            offset: 0.,
        }
    }
}

impl Device for Clock {
    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    fn closest_border_point(&self, point: Vec2, padding: f32) -> Vec2 {
        let delta = point - self.position;
        self.position + delta.normalize() * (CLOCK_RADIUS + padding)
    }

    fn is_point_inside(&self, pt: Vec2) -> bool {
        self.position.distance(pt) <= CLOCK_RADIUS
    }

    fn draw(&self) {
        let Vec2 { x, y } = self.position;
        draw_circle_lines(x, y, CLOCK_RADIUS, 1.0, WHITE);
    }

    fn input_arity(&self) -> Arity {
        Arity::Nullary
    }

    fn has_output(&self) -> bool {
        true
    }
}
