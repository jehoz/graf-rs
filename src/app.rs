use macroquad::{
    color::{BLACK, WHITE},
    input::{is_mouse_button_pressed, is_mouse_button_released, mouse_position, MouseButton},
    math::{vec2, Vec2},
    shapes::draw_line,
    ui::{hash, root_ui, widgets::Window},
    window::clear_background,
};

use crate::{
    dag::VertexId,
    devices::{Clock, Gate, Note},
    session::Session,
};

enum CursorState {
    HoveringNothing,
    DraggingDevice(VertexId),
    DraggingLooseWire(VertexId),
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
        let (mx, my) = mouse_position();
        let m_pos = vec2(mx, my);
        let device_under_mouse = self.session.get_device_at(m_pos);

        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some(id) = device_under_mouse {
                self.cursor = CursorState::DraggingDevice(id);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.cursor = CursorState::HoveringNothing;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            match device_under_mouse {
                Some(id) => self.cursor = CursorState::DraggingLooseWire(id),
                None => self.context_menu = Some(m_pos),
            }
        }

        // update
        match self.cursor {
            CursorState::HoveringNothing => {}
            CursorState::DraggingDevice(id) => {
                self.session.move_device(id, m_pos);
            }
            CursorState::DraggingLooseWire(from_id)
            | CursorState::DraggingConnectedWire(from_id, _) => match device_under_mouse {
                Some(to_id) => self.cursor = CursorState::DraggingConnectedWire(from_id, to_id),
                None => self.cursor = CursorState::DraggingLooseWire(from_id),
            },
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
        clear_background(BLACK);

        // draw new wire being dragged out if there is one
        if let CursorState::DraggingLooseWire(id) = self.cursor {
            let dev_pos = self.session.device_position(id).unwrap();
            let (mx, my) = mouse_position();
            draw_line(dev_pos.x, dev_pos.y, mx, my, 1.0, WHITE);
        } else if let CursorState::DraggingConnectedWire(from_id, to_id) = self.cursor {
            let from_pos = self.session.device_position(from_id).unwrap();
            let to_pos = self.session.device_position(to_id).unwrap();
            draw_line(from_pos.x, from_pos.y, to_pos.x, to_pos.y, 1.0, WHITE);
        }

        self.session.draw();
    }
}
