use std::time::Duration;

use macroquad::{
    math::Vec2,
    shapes::{draw_circle, draw_circle_lines}
};

use egui::{Slider, FontId, RichText};

use crate::devices::{Arity, Device, TRIGGER_RADIUS};

#[derive(Clone)]
pub struct Trigger {
    position: Vec2,

    // duration in milliseconds that trigger will stay on after being set off
    duration: f32,

    // can the trigger be set off again before finishing?
    retrigger_mode: bool,

    ready_to_fire: bool,
    time_remaining: Option<f32>,

    prev_clock_time: Duration,
}

impl Trigger {
    pub fn new(position: Vec2) -> Self {
        Trigger {
            position,
            duration: 500.0,
            retrigger_mode: true,

            ready_to_fire: true,
            time_remaining: None,

            prev_clock_time: Duration::ZERO,
        }
    }

    fn fire(&mut self) {
        self.ready_to_fire = false;
        self.time_remaining = Some(self.duration);
    }
}

impl Device for Trigger {
    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, pos: Vec2) {
        self.position = pos;
    }

    fn closest_border_point(&self, point: Vec2, padding: f32) -> Vec2 {
        let delta = point - self.position;
        self.position + delta.normalize() * (TRIGGER_RADIUS + padding)
    }

    fn is_point_inside(&self, pt: Vec2) -> bool {
        self.position.distance(pt) <= TRIGGER_RADIUS
    }

    fn update(&mut self, ctx: &mut crate::session::UpdateContext, inputs: Vec<bool>) -> Option<bool> {
        let input_on = inputs.first().map(|x| *x).unwrap_or(false);
        if self.retrigger_mode {
            if input_on && self.ready_to_fire {
                self.fire();
            }
        } else {
            if input_on && self.ready_to_fire && self.time_remaining == None {
                self.fire();
            }
        }

        if !input_on {
            self.ready_to_fire = true;
        }

        let delta_t = ctx.free_clock - self.prev_clock_time;
        self.prev_clock_time = ctx.free_clock;

        if let Some(t_prev) = self.time_remaining {
            let t = t_prev - delta_t.as_secs_f32() * 1000.0;
            if t > 0.0 {
                self.time_remaining = Some(t);
            } else {
                self.time_remaining = None;
            }
            Some(true)
        } else {
            Some(false)
        }

    }

    fn draw(&self, ctx: &crate::session::DrawContext, is_selected: bool) {
        let Vec2 { x, y } = ctx.world_to_viewport(self.position);

        if is_selected {
            draw_circle_lines(x, y, TRIGGER_RADIUS + 4.0, 2.0, ctx.fg_color.with_alpha(0.5));
        }

        draw_circle_lines(x, y, TRIGGER_RADIUS, 1.0, ctx.fg_color);
        draw_circle(x, y, TRIGGER_RADIUS, ctx.bg_color);

        if let Some(t_rem) = self.time_remaining {
            let percent_done = (t_rem / self.duration).clamp(0.0, 1.0);
            draw_circle(x, y, TRIGGER_RADIUS * percent_done, ctx.fg_color);
        }
    }

    fn inspector(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("Trigger")
                .font(FontId::proportional(16.0))
                .strong(),
        );
        ui.separator();

        ui.checkbox(&mut self.retrigger_mode, "Retrigger Mode");
        
        ui.add(Slider::new(&mut self.duration, 1f32..=10000f32).text("Duration").suffix("ms"));
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
