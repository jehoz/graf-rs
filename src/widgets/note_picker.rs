use egui::{Align2, FontId, Grid, Pos2, Rect, Sense, Shape, Stroke, StrokeKind, Vec2, Widget};

use crate::devices::note::PitchClass;

pub struct NotePicker<'a> {
    note: &'a mut PitchClass,

    width: f32,
    height: f32,
    key_corner_radius: f32,
}

const WHITE_KEYS: [PitchClass; 7] = [
    PitchClass::C,
    PitchClass::D,
    PitchClass::E,
    PitchClass::F,
    PitchClass::G,
    PitchClass::A,
    PitchClass::B,
];

const BLACK_KEYS: [PitchClass; 5] = [
    PitchClass::Cs,
    PitchClass::Ds,
    PitchClass::Fs,
    PitchClass::Gs,
    PitchClass::As,
];

impl<'a> NotePicker<'a> {
    pub fn new(var: &'a mut PitchClass) -> NotePicker<'a> {
        NotePicker {
            note: var,
            width: 200.0,
            height: 80.0,
            key_corner_radius: 2.0,
        }
    }
}

impl<'a> Widget for NotePicker<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let desired_size = Vec2::new(self.width, self.height);
        let response = ui.allocate_response(desired_size, Sense::click());

        let key_width = response.rect.width() / 7.0;
        if ui.is_rect_visible(response.rect) {
            let mut key_clicked = None;

            // white keys
            for (i, key) in WHITE_KEYS.iter().enumerate() {
                let key_rect = Rect::from_center_size(
                    Pos2::new(
                        response.rect.left() + (i as f32 + 0.5) * key_width,
                        response.rect.center().y,
                    ),
                    Vec2::new(key_width, response.rect.height()),
                );

                let fill_color = if *self.note == *key {
                    ui.visuals().text_color()
                } else {
                    ui.visuals().weak_text_color()
                };

                ui.painter().add(Shape::rect_filled(
                    key_rect,
                    self.key_corner_radius,
                    fill_color,
                ));
                ui.painter().add(Shape::rect_stroke(
                    key_rect,
                    self.key_corner_radius,
                    Stroke::new(1.0, ui.visuals().extreme_bg_color),
                    StrokeKind::Inside,
                ));
                ui.painter().text(
                    key_rect.center_bottom() - Vec2::new(0.0, 5.0),
                    Align2::CENTER_BOTTOM,
                    *key,
                    FontId::proportional(11.0),
                    ui.visuals().extreme_bg_color,
                );

                if let Some(pos) = response.interact_pointer_pos() {
                    if key_rect.contains(pos) {
                        key_clicked = Some(*key);
                    }
                }
            }

            // black keys
            for (i, key) in BLACK_KEYS.iter().take(2).enumerate() {
                let key_rect = Rect::from_center_size(
                    Pos2::new(
                        response.rect.left() + (i as f32 + 1.0) * key_width,
                        response.rect.center().y - (response.rect.height() * 0.25 * 0.67),
                    ),
                    Vec2::new(key_width * 0.75, response.rect.height() * 0.67),
                );

                let fill_color = if *self.note == *key {
                    ui.visuals().text_color()
                } else {
                    ui.visuals().weak_text_color()
                };

                ui.painter().add(Shape::rect_filled(
                    key_rect,
                    self.key_corner_radius,
                    fill_color,
                ));
                ui.painter().add(Shape::rect_stroke(
                    key_rect,
                    self.key_corner_radius,
                    Stroke::new(2.0, ui.visuals().extreme_bg_color),
                    StrokeKind::Inside,
                ));
                ui.painter().text(
                    key_rect.center_bottom() - Vec2::new(0.0, 5.0),
                    Align2::CENTER_BOTTOM,
                    *key,
                    FontId::proportional(11.0),
                    ui.visuals().extreme_bg_color,
                );

                if let Some(pos) = response.interact_pointer_pos() {
                    if key_rect.contains(pos) {
                        key_clicked = Some(*key);
                    }
                }
            }
            for (i, key) in BLACK_KEYS.iter().skip(2).enumerate() {
                let key_rect = Rect::from_center_size(
                    Pos2::new(
                        response.rect.left() + (i as f32 + 4.0) * key_width,
                        response.rect.center().y - (response.rect.height() * 0.25 * 0.67),
                    ),
                    Vec2::new(key_width * 0.75, response.rect.height() * 0.67),
                );

                let fill_color = if *self.note == *key {
                    ui.visuals().text_color()
                } else {
                    ui.visuals().weak_text_color()
                };

                ui.painter().add(Shape::rect_filled(
                    key_rect,
                    self.key_corner_radius,
                    fill_color,
                ));
                ui.painter().add(Shape::rect_stroke(
                    key_rect,
                    self.key_corner_radius,
                    Stroke::new(2.0, ui.visuals().extreme_bg_color),
                    StrokeKind::Inside,
                ));
                ui.painter().text(
                    key_rect.center_bottom() - Vec2::new(0.0, 5.0),
                    Align2::CENTER_BOTTOM,
                    *key,
                    FontId::proportional(11.0),
                    ui.visuals().extreme_bg_color,
                );

                if let Some(pos) = response.interact_pointer_pos() {
                    if key_rect.contains(pos) {
                        key_clicked = Some(*key);
                    }
                }
            }

            if let Some(key) = key_clicked {
                *self.note = key;
            }
        }

        response
    }
}
