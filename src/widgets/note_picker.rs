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

        let key_width = response.rect.width() / 8.0;
        if ui.is_rect_visible(response.rect) {
            // white keys
            for i in 0..=7 {
                let key_rect = Rect::from_min_max(
                    Pos2::new(i as f32 * key_width, 0.0),
                    Pos2::new((i + 1) as f32 * key_width, response.rect.height()),
                );
                let key = Shape::rect_stroke(
                    key_rect,
                    self.key_corner_radius,
                    Stroke::new(1.0, ui.visuals().text_color()),
                    StrokeKind::Outside,
                );
                ui.painter().add(key);
            }
        }

        response
    }
}
