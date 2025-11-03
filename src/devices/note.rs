use core::ops::Drop;

use egui::{DragValue, FontId, RichText};
use macroquad::{
    math::Vec2,
    shapes::{draw_circle, draw_circle_lines},
};
use midly::live::LiveEvent;

use crate::{
    midi::MidiConfig,
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

#[derive(Clone)]
pub struct Note {
    position: Vec2,

    midi_channel: u8,
    octave: u8,
    pitch_class: PitchClass,
    velocity: u8,

    is_on: bool,
    // use this outside of update so that we can clean up the midi state
    // properly next time update is called
    should_change_note: Option<(u8, PitchClass)>,
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
            should_change_note: None,
        }
    }

    fn midi_key(&self) -> u8 {
        self.pitch_class as u8 + self.octave * 12
    }

    fn turn_on(&mut self, midi: &mut MidiConfig) {
        if self.is_on {
            return;
        }

        let event = LiveEvent::Midi {
            channel: self.midi_channel.into(),
            message: midly::MidiMessage::NoteOn {
                key: self.midi_key().into(),
                vel: self.velocity.into(),
            },
        };

        midi.handle_live_event(event);
        self.is_on = true;
    }

    fn turn_off(&mut self, midi: &mut MidiConfig) {
        if !self.is_on {
            return;
        }

        let event = LiveEvent::Midi {
            channel: self.midi_channel.into(),
            message: midly::MidiMessage::NoteOff {
                key: self.midi_key().into(),
                vel: self.velocity.into(),
            },
        };

        midi.handle_live_event(event);
        self.is_on = false;
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
        if let Some((octave, pitch)) = self.should_change_note {
            self.turn_off(&mut ctx.midi_config);
            self.octave = octave;
            self.pitch_class = pitch;
            self.should_change_note = None;
        }

        if let Some(input_on) = inputs.first() {
            if *input_on {
                self.turn_on(&mut ctx.midi_config);
            } else {
                self.turn_off(&mut ctx.midi_config);
            }
        } else {
            self.turn_off(&mut ctx.midi_config);
        }
        None
    }

    fn draw(&self, ctx: &DrawContext, is_selected: bool) {
        let Vec2 { x, y } = ctx.world_to_viewport(self.position);

        if is_selected {
            draw_circle_lines(x, y, NOTE_RADIUS + 4.0, 2.0, ctx.fg_color.with_alpha(0.5));
        }

        draw_circle_lines(x, y, NOTE_RADIUS, 1.0, ctx.fg_color);
        if self.is_on {
            draw_circle(x, y, NOTE_RADIUS / 2.0, ctx.fg_color);
        }
    }

    fn inspector(&mut self, ui: &mut egui::Ui) {
        let mut octave = self.octave;
        let mut pitch = self.pitch_class;

        ui.label(
            RichText::new("Note")
                .font(FontId::proportional(16.0))
                .strong(),
        );
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Octave");

            let btn_size = egui::vec2(20.0, 20.0);
            let dec_btn = ui.add_sized(btn_size, egui::Button::new("-"));
            if dec_btn.clicked() {
                octave -= 1;
            }

            ui.add(DragValue::new(&mut octave).range(0..=8));

            let inc_btn = ui.add_sized(btn_size, egui::Button::new("+"));
            if inc_btn.clicked() {
                octave += 1;
            }
        });

        ui.add_space(2.0);

        ui.add(NotePicker::new(&mut pitch));

        if self.octave != octave || self.pitch_class != pitch {
            self.should_change_note = Some((octave, pitch));
        }

        ui.add(egui::Slider::new(&mut self.velocity, 0..=127).text("Velocity"));
    }

    fn input_arity(&self) -> Arity {
        Arity::Unary
    }

    fn has_output(&self) -> bool {
        false
    }

    fn clone_dyn(&self) -> Box<dyn Device> {
        Box::new(self.clone())
    }
}
