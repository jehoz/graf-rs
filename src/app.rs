use core::panic;

use egui::Align2;
use macroquad::{
    input::{
        is_key_pressed, is_mouse_button_pressed, is_mouse_button_released, mouse_position, KeyCode,
        MouseButton,
    },
    math::{vec2, Rect, Vec2},
    shapes::draw_rectangle_lines,
    ui::{hash, root_ui, widgets::Window},
    window::clear_background,
};

use crate::{
    dag::DeviceId,
    devices::{clock::Clock, gate::Gate, note::Note},
    drawing_utils::{draw_wire_between_devices, draw_wire_from_device},
    session::Session,
};

enum CursorState {
    Idle,
    DraggingSelectedDevices(Vec2),
    DraggingLooseWire(DeviceId),
    DraggingConnectedWire(DeviceId, DeviceId),
    DraggingInvalidWire(DeviceId),
    DraggingSelectBox(Vec2),
}

const INSPECTOR_WIDTH: f32 = 200.0;

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
                        if !self.session.selected.contains(&id) {
                            self.session.clear_selection();
                            self.session.select_device(id);
                        }
                        self.cursor = CursorState::DraggingSelectedDevices(m_pos);
                    }

                    if is_mouse_button_pressed(MouseButton::Right) {
                        self.cursor = CursorState::DraggingLooseWire(id);
                    }
                }
                None => {
                    if is_mouse_button_pressed(MouseButton::Right) {
                        let wire_under_mouse = self.session.get_wire_at(m_pos);

                        match wire_under_mouse {
                            Some((from_id, to_id)) => {
                                self.session.disconnect_devices(from_id, to_id);
                                self.cursor = CursorState::DraggingLooseWire(from_id);
                            }
                            None => {
                                self.context_menu = Some(m_pos);
                            }
                        }
                    }
                    if is_mouse_button_pressed(MouseButton::Left) {
                        self.cursor = CursorState::DraggingSelectBox(m_pos);
                    }
                }
            },

            CursorState::DraggingSelectedDevices(from) => {
                self.session.move_selected_devices(m_pos - from);
                self.cursor = CursorState::DraggingSelectedDevices(m_pos);

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
            CursorState::DraggingSelectBox(starting_corner) => {
                if is_mouse_button_released(MouseButton::Left) {
                    self.cursor = CursorState::Idle;
                } else {
                    let top = f32::min(starting_corner.y, m_pos.y);
                    let left = f32::min(starting_corner.x, m_pos.x);
                    let delta = (m_pos - starting_corner).abs();
                    let rect = Rect::new(left, top, delta.x, delta.y);

                    self.session.clear_selection();
                    self.session.select_devices_in_rect(rect);
                }
            }
        }

        if is_key_pressed(KeyCode::Delete) {
            self.session.delete_selected_devices();
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

    pub fn update(&mut self) {
        self.session.update();
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if let [selected_id] = self.session.selected.as_slice() {
            match self.session.devices.get_mut(&selected_id) {
                Some(dev) => {
                    egui::Window::new("Edit Device")
                        .anchor(Align2::RIGHT_TOP, [-10.0, 10.0])
                        .movable(false)
                        .title_bar(false)
                        .default_width(INSPECTOR_WIDTH)
                        .resizable(false)
                        .show(ctx, |ui| dev.inspector(ui));
                }
                None => {
                    panic!("Tried to inspect device that doesn't exist???")
                }
            }
        }
    }

    pub fn draw(&self) {
        let (mx, my) = mouse_position();
        let m_pos = vec2(mx, my);

        clear_background(self.session.draw_ctx.bg_color);

        match self.cursor {
            CursorState::Idle | CursorState::DraggingSelectedDevices(_) => {}
            CursorState::DraggingLooseWire(from_id) => {
                let from_dev = self.session.devices.get(&from_id).unwrap();
                draw_wire_from_device(from_dev.as_ref(), m_pos, self.session.draw_ctx.fg_color);
            }
            CursorState::DraggingConnectedWire(from_id, to_id) => {
                let from_dev = self.session.devices.get(&from_id).unwrap();
                let to_dev = self.session.devices.get(&to_id).unwrap();
                draw_wire_between_devices(
                    from_dev.as_ref(),
                    to_dev.as_ref(),
                    self.session.draw_ctx.fg_color,
                );
            }
            CursorState::DraggingInvalidWire(from_id) => {
                let from_dev = self.session.devices.get(&from_id).unwrap();
                draw_wire_from_device(from_dev.as_ref(), m_pos, self.session.draw_ctx.err_color);
            }
            CursorState::DraggingSelectBox(starting_corner) => {
                let top = f32::min(starting_corner.y, m_pos.y);
                let left = f32::min(starting_corner.x, m_pos.x);
                let delta = (m_pos - starting_corner).abs();
                draw_rectangle_lines(
                    left,
                    top,
                    delta.x,
                    delta.y,
                    1.0,
                    self.session.draw_ctx.fg_color,
                );
            }
        }

        self.session.draw();
    }
}
