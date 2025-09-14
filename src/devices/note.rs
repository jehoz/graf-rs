use egui::DragValue;
use macroquad::{
    math::Vec2,
    shapes::{draw_circle, draw_circle_lines},
    ui::hash,
};
use midly::live::LiveEvent;

use crate::session::{DrawContext, UpdateContext};

use super::{Arity, Device, NOTE_RADIUS};

#[derive(Clone, Copy)]
enum OctaveNote {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

pub struct Note {
    position: Vec2,

    midi_channel: u8,
    octave: u8,
    note: OctaveNote,
    velocity: u8,

    is_on: bool,
}

impl Note {
    pub fn new(position: Vec2) -> Self {
        Note {
            position,

            midi_channel: 0,
            octave: 4,
            note: OctaveNote::C,
            velocity: 100,

            is_on: false,
        }
    }

    fn midi_key(&self) -> u8 {
        self.note as u8 + self.octave * 12
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

    fn update(&mut self, ctx: &mut UpdateContext, inputs: Vec<bool>) -> Option<bool> {
        if let Some(input_on) = inputs.first() {
            if *input_on && !self.is_on {
                let event = LiveEvent::Midi {
                    channel: self.midi_channel.into(),
                    message: midly::MidiMessage::NoteOn {
                        key: self.midi_key().into(),
                        vel: self.velocity.into(),
                    },
                };

                ctx.midi_config.handle_live_event(event);
                self.is_on = true;
            } else if !(*input_on) && self.is_on {
                let event = LiveEvent::Midi {
                    channel: self.midi_channel.into(),
                    message: midly::MidiMessage::NoteOff {
                        key: self.midi_key().into(),
                        vel: self.velocity.into(),
                    },
                };

                ctx.midi_config.handle_live_event(event);
                self.is_on = false;
            }
        }
        None
    }

    fn draw(&self, ctx: &DrawContext) {
        let Vec2 { x, y } = self.position;
        draw_circle_lines(x, y, NOTE_RADIUS, 1.0, ctx.fg_color);
        if self.is_on {
            draw_circle(x, y, NOTE_RADIUS / 2.0, ctx.fg_color);
        }
    }

    fn inspector(&mut self, ui: &mut egui::Ui) {
        ui.label("Edit Note");
        ui.separator();
        ui.add(DragValue::new(&mut self.octave).range(0..=8));
    }

    fn input_arity(&self) -> Arity {
        Arity::Unary
    }

    fn has_output(&self) -> bool {
        false
    }
}
