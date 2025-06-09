use std::time::Instant;

use macroquad::{
    color::{BLACK, RED, WHITE},
    math::Vec2,
    shapes::{draw_arc, draw_circle, draw_circle_lines},
};

use super::{Arity, Device, CLOCK_RADIUS};

pub enum Period {
    Milliseconds(f32),
    Beats(BeatFraction),
}

pub struct BeatFraction {
    numerator: u16,
    denominator: u16,
}

pub struct Clock {
    position: Vec2,

    period: Period,
    duty_cycle: f32,
    offset: f32,

    last_timestamp: Instant,
    cycle_position: f32,
}

impl Clock {
    pub fn new(position: Vec2) -> Self {
        Clock {
            position,
            period: Period::Beats(BeatFraction {
                numerator: 1,
                denominator: 4,
            }),
            duty_cycle: 0.5,
            offset: 0.,

            last_timestamp: Instant::now(),
            cycle_position: 0.0,
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

    fn update(&mut self, _inputs: Vec<bool>) -> Option<bool> {
        let now = Instant::now();
        let dt = now.duration_since(self.last_timestamp);

        // let dc = (1000.0 * dt.as_secs_f32()) / self.period
        let dc = (1000.0 * dt.as_secs_f32()) / 1500.0;

        self.cycle_position = (self.cycle_position + dc) % 1.0;

        if (self.cycle_position - self.offset) % 1.0 <= self.duty_cycle {
            Some(true)
        } else {
            Some(false)
        }
    }

    fn draw(&self) {
        let Vec2 { x, y } = self.position;
        draw_circle_lines(x, y, CLOCK_RADIUS, 1.0, WHITE);
        draw_circle(x, y, CLOCK_RADIUS, BLACK);

        draw_arc(
            x,
            y,
            32,
            0.0,
            360.0 * self.cycle_position,
            CLOCK_RADIUS,
            360.0 * self.duty_cycle,
            WHITE,
        );
    }

    fn input_arity(&self) -> Arity {
        Arity::Nullary
    }

    fn has_output(&self) -> bool {
        true
    }
}
