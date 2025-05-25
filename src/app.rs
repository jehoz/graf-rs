use macroquad::{
    color::BLACK,
    input::{is_mouse_button_pressed, is_mouse_button_released, mouse_position, MouseButton},
    math::{vec2, Vec2},
    ui::{hash, root_ui, widgets::Window},
    window::clear_background,
};

use crate::{dag::VertexId, session::Session};

enum CursorState {
    HoveringNothing,
    DraggingDevice(VertexId),
    DraggingLooseWire(VertexId, Vec2),
    DraggingConnectedWire(VertexId, VertexId),
}

pub struct App {
    session: Session,
    cursor: CursorState,

    context_menu: Option<Vec2>,
}

impl App {
    pub fn new() -> Self {
        App {
            session: Session::new(),
            cursor: CursorState::HoveringNothing,

            context_menu: None,
        }
    }

    pub fn handle_inputs(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            match self.session.get_device_at(vec2(mx, my)) {
                Some(vid) => self.cursor = CursorState::DraggingDevice(vid),
                None => {}
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.cursor = CursorState::HoveringNothing;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let (mx, my) = mouse_position();
            self.context_menu = Some(vec2(mx, my));
        }

        // update
        if let CursorState::DraggingDevice(id) = self.cursor {
            let (mx, my) = mouse_position();
            self.session.move_device(id, vec2(mx, my));
        }

        if let Some(pos) = self.context_menu {
            Window::new(hash!(), pos, vec2(60., 90.))
                .label("New device")
                .movable(false)
                .ui(&mut *root_ui(), |ui| {
                    if ui.button(None, "Clock") {
                        self.session.create_clock(pos);
                        self.context_menu = None;
                    }
                    if ui.button(None, "Gate") {
                        self.session.create_gate(pos);
                        self.context_menu = None;
                    }
                    if ui.button(None, "Note") {
                        self.session.create_note(pos);
                        self.context_menu = None;
                    }
                });
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);
        self.session.draw();
    }
}
