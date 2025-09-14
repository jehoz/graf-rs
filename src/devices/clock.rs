use std::time::Instant;

use egui::{DragValue, Slider};
use macroquad::{
    math::Vec2,
    shapes::{draw_arc, draw_circle, draw_circle_lines},
};

use crate::session::{DrawContext, UpdateContext};

use super::{Arity, Device, CLOCK_RADIUS};

pub enum Period {
    Milliseconds(f32),
    NoteLength { numerator: u32, denominator: u32 },
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
            period: Period::NoteLength {
                numerator: 1,
                denominator: 4,
            },
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

    fn update(&mut self, ctx: &mut UpdateContext, _inputs: Vec<bool>) -> Option<bool> {
        let now = Instant::now();
        let time_ms = now.duration_since(ctx.t0).as_secs_f32() * 1000.0;

        let period_ms = match self.period {
            Period::Milliseconds(ms) => ms,
            Period::NoteLength {
                numerator,
                denominator,
            } => {
                let ms_per_note = 240_000.0 / (ctx.bpm as f32);
                (numerator as f32 / denominator as f32) * ms_per_note
            }
        };

        self.cycle_position = ((time_ms + self.offset * period_ms) % period_ms) / period_ms;

        if self.cycle_position <= self.duty_cycle {
            Some(true)
        } else {
            Some(false)
        }
    }

    fn draw(&self, ctx: &DrawContext) {
        let Vec2 { x, y } = self.position;
        draw_circle_lines(x, y, CLOCK_RADIUS, 1.0, ctx.fg_color);
        draw_circle(x, y, CLOCK_RADIUS, ctx.bg_color);

        draw_arc(
            x,
            y,
            32,
            0.0,
            360.0 * self.cycle_position,
            CLOCK_RADIUS,
            360.0 * self.duty_cycle,
            ctx.fg_color,
        );
    }

    fn inspector(&mut self, ui: &mut egui::Ui) {
        ui.label("Edit Clock");
        ui.separator();
        match &mut self.period {
            Period::Milliseconds(ms) => {
                ui.add(Slider::new(ms, 1f32..=10000f32).text("Period"));
            }
            Period::NoteLength {
                numerator,
                denominator,
            } => {
                ui.horizontal(|ui| {
                    ui.label("Note Length");
                    ui.add(DragValue::new(numerator).range(1..=256));
                    ui.label("/");
                    ui.add(DragValue::new(denominator).range(1..=256));
                });
            }
        }

        ui.separator();
        ui.add(Slider::new(&mut self.duty_cycle, 0f32..=1.0f32).text("Duty Cycle"));

        ui.separator();
        ui.add(Slider::new(&mut self.offset, 0f32..=1.0f32).text("Offset"));
    }

    fn input_arity(&self) -> Arity {
        Arity::Nullary
    }

    fn has_output(&self) -> bool {
        true
    }
}
