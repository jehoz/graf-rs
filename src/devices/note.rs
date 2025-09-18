use egui::{DragValue, FontId, RichText};
use macroquad::{
    math::Vec2,
    shapes::{draw_circle, draw_circle_lines},
};
use midly::live::LiveEvent;

use crate::{
    session::{DrawContext, UpdateContext},
    widgets::note_picker::NotePicker,
};

use super::{Arity, Device, NOTE_RADIUS};

#[derive(Clone, Copy, PartialEq)]
pub enum PitchClass {
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

impl ToString for PitchClass {
    fn to_string(&self) -> String {
        match *self {
            PitchClass::C => "C".to_string(),
            PitchClass::Cs => "C#".to_string(),
            PitchClass::D => "D".to_string(),
            PitchClass::Ds => "D#".to_string(),
            PitchClass::E => "E".to_string(),
            PitchClass::F => "F".to_string(),
            PitchClass::Fs => "F#".to_string(),
            PitchClass::G => "G".to_string(),
            PitchClass::Gs => "G#".to_string(),
            PitchClass::A => "A".to_string(),
            PitchClass::As => "A#".to_string(),
            PitchClass::B => "B".to_string(),
        }
    }
}

pub struct Note {
    position: Vec2,

    midi_channel: u8,
    octave: u8,
    pitch_class: PitchClass,
    velocity: u8,

    is_on: bool,
}

impl Note {
    pub fn new(position: Vec2) -> Self {
        Note {
            position,

            midi_channel: 0,
            octave: 4,
            pitch_class: PitchClass::C,
            velocity: 100,

            is_on: false,
        }
    }

    fn midi_key(&self) -> u8 {
        self.pitch_class as u8 + self.octave * 12
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
        ui.label(
            RichText::new("Note")
                .font(FontId::proportional(16.0))
                .strong(),
        );
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Octave");
            ui.add(DragValue::new(&mut self.octave).range(0..=8));
        });

        ui.add_space(2.0);

        ui.add(NotePicker::new(&mut self.pitch_class));
    }

    fn input_arity(&self) -> Arity {
        Arity::Unary
    }

    fn has_output(&self) -> bool {
        false
    }
}
