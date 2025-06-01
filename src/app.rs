use macroquad::{
    color::{BLACK, RED, WHITE},
    input::{is_mouse_button_pressed, is_mouse_button_released, mouse_position, MouseButton},
    math::{vec2, Vec2},
    ui::{hash, root_ui, widgets::Window},
    window::clear_background,
};

use crate::{
    dag::VertexId,
    devices::{clock::Clock, gate::Gate, note::Note},
    drawing_utils::{draw_wire_between_devices, draw_wire_from_device},
    session::Session,
};

enum CursorState {
    Idle,
    DraggingDevice(VertexId),
    DraggingLooseWire(VertexId),
    DraggingConnectedWire(VertexId, VertexId),
    DraggingInvalidWire(VertexId),
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
            cursor: CursorState::Idle,

            context_menu: None,
        }
    }

    pub fn handle_inputs(&mut self) {
        let (mx, my) = mouse_position();
        let m_pos = vec2(mx, my);
        let device_under_mouse = self.session.get_device_at(m_pos);

        match self.cursor {
            CursorState::Idle => match device_under_mouse {
                Some(id) => {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        self.cursor = CursorState::DraggingDevice(id);
                    }
                    if is_mouse_button_pressed(MouseButton::Right) {
                        self.cursor = CursorState::DraggingLooseWire(id);
                    }
                }
                None => {
                    if is_mouse_button_pressed(MouseButton::Right) {
                        self.context_menu = Some(m_pos);
                    }
                }
            },

            CursorState::DraggingDevice(id) => {
                self.session.move_device(id, m_pos);
                if is_mouse_button_released(MouseButton::Left) {
                    self.cursor = CursorState::Idle;
                }
            }

            CursorState::DraggingLooseWire(from_id) => {
                if is_mouse_button_released(MouseButton::Right) {
                    self.cursor = CursorState::Idle;
                } else if let Some(to_id) = device_under_mouse {
                    if self.session.can_connect(from_id, to_id) {
                        self.cursor = CursorState::DraggingConnectedWire(from_id, to_id);
                    } else {
                        self.cursor = CursorState::DraggingInvalidWire(from_id);
                    }
                }
            }

            CursorState::DraggingConnectedWire(from_id, to_id) => {
                if is_mouse_button_released(MouseButton::Right) {
                    self.session.connect_devices(from_id, to_id);
                    self.cursor = CursorState::Idle;
                } else {
                    match device_under_mouse {
                        Some(to_id) => {
                            if !self.session.can_connect(from_id, to_id) {
                                self.cursor = CursorState::DraggingInvalidWire(from_id);
                            }
                        }
                        None => self.cursor = CursorState::DraggingLooseWire(from_id),
                    }
                }
            }

            CursorState::DraggingInvalidWire(from_id) => {
                if is_mouse_button_released(MouseButton::Right) {
                    self.cursor = CursorState::Idle;
                } else {
                    match device_under_mouse {
                        Some(to_id) => {
                            if self.session.can_connect(from_id, to_id) {
                                self.cursor = CursorState::DraggingConnectedWire(from_id, to_id);
                            }
                        }
                        None => self.cursor = CursorState::DraggingLooseWire(from_id),
                    }
                }
            }
        }

        if let Some(pos) = self.context_menu {
            Window::new(hash!(), pos, vec2(60., 90.))
                .label("New device")
                .movable(false)
                .ui(&mut *root_ui(), |ui| {
                    if ui.button(None, "Clock") {
                        let clock = Clock::new(pos);
                        self.session.add_device(Box::new(clock));
                        self.context_menu = None;
                    }
                    if ui.button(None, "Gate") {
                        let gate = Gate::new(pos);
                        self.session.add_device(Box::new(gate));
                        self.context_menu = None;
                    }
                    if ui.button(None, "Note") {
                        let note = Note::new(pos);
                        self.session.add_device(Box::new(note));
                        self.context_menu = None;
                    }
                });
        }
    }

    pub fn draw(&self) {
        let (mx, my) = mouse_position();
        let m_pos = vec2(mx, my);

        clear_background(BLACK);

        // draw wire being dragged out if there is one
        match self.cursor {
            CursorState::Idle | CursorState::DraggingDevice(_) => {}
            CursorState::DraggingLooseWire(from_id) => {
                let from_dev = self.session.devices.get(&from_id).unwrap();
                draw_wire_from_device(from_dev.as_ref(), m_pos, WHITE);
            }
            CursorState::DraggingConnectedWire(from_id, to_id) => {
                let from_dev = self.session.devices.get(&from_id).unwrap();
                let to_dev = self.session.devices.get(&to_id).unwrap();
                draw_wire_between_devices(from_dev.as_ref(), to_dev.as_ref(), WHITE);
            }
            CursorState::DraggingInvalidWire(from_id) => {
                let from_dev = self.session.devices.get(&from_id).unwrap();
                draw_wire_from_device(from_dev.as_ref(), m_pos, RED);
            }
        }

        self.session.draw();
    }
}
