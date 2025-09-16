use egui::{Pos2, Rect, Sense, Shape, Stroke, StrokeKind, Vec2, Widget};

use crate::devices::note::PitchClass;

pub struct NotePicker<'a> {
    note: &'a mut PitchClass,

    width: f32,
    key_corner_radius: f32,
}

impl<'a> NotePicker<'a> {
    pub fn new(var: &'a mut PitchClass) -> NotePicker<'a> {
        NotePicker {
            note: var,
            width: 200.0,
            key_corner_radius: 2.0,
        }
    }
}

impl<'a> Widget for NotePicker<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let desired_size = Vec2::new(self.width, self.width * 0.5);
        let mut response = ui.allocate_response(desired_size, Sense::click());

        let key_width = response.rect.width() / 7.0;
        if ui.is_rect_visible(response.rect) {
            // white keys
            for i in 0..7 {
                let key_rect = Rect::from_center_size(
                    Pos2::new(
                        response.rect.left() + (i as f32 + 0.5) * key_width,
                        response.rect.center().y,
                    ),
                    Vec2::new(key_width, response.rect.height()),
                );

                ui.painter().add(Shape::rect_filled(
                    key_rect,
                    self.key_corner_radius,
                    ui.visuals().weak_text_color(),
                ));
                ui.painter().add(Shape::rect_stroke(
                    key_rect,
                    self.key_corner_radius,
                    Stroke::new(1.0, ui.visuals().extreme_bg_color),
                    StrokeKind::Inside,
                ));
            }

            // black keys
            for i in 0..2 {
                let key_rect = Rect::from_center_size(
                    Pos2::new(
                        response.rect.left() + (i as f32 + 1.0) * key_width,
                        response.rect.center().y - (response.rect.height() * 0.25 * 0.67),
                    ),
                    Vec2::new(key_width * 0.67, response.rect.height() * 0.67),
                );

                ui.painter().add(Shape::rect_filled(
                    key_rect,
                    self.key_corner_radius,
                    ui.visuals().weak_text_color(),
                ));
                ui.painter().add(Shape::rect_stroke(
                    key_rect,
                    self.key_corner_radius,
                    Stroke::new(2.0, ui.visuals().extreme_bg_color),
                    StrokeKind::Inside,
                ));
            }
            for i in 0..3 {
                let key_rect = Rect::from_center_size(
                    Pos2::new(
                        response.rect.left() + (i as f32 + 4.0) * key_width,
                        response.rect.center().y - (response.rect.height() * 0.25 * 0.67),
                    ),
                    Vec2::new(key_width * 0.67, response.rect.height() * 0.67),
                );

                ui.painter().add(Shape::rect_filled(
                    key_rect,
                    self.key_corner_radius,
                    ui.visuals().weak_text_color(),
                ));
                ui.painter().add(Shape::rect_stroke(
                    key_rect,
                    self.key_corner_radius,
                    Stroke::new(2.0, ui.visuals().extreme_bg_color),
                    StrokeKind::Inside,
                ));
            }
        }

        response
    }
}
