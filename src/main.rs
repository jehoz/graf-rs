use dag::{Edge, VertexId};
use macroquad::{
    prelude::*,
    ui::{
        hash, root_ui,
        widgets::{Group, Window},
    },
};
use session::Session;

mod dag;
mod devices;
mod session;

const NODE_SIZE: f32 = 50.0;
const HALF_NODE_SIZE: f32 = NODE_SIZE / 2.0;

enum CursorState {
    HoveringNothing,
    HoveringDevice(VertexId),
    DraggingDevice(VertexId),
    DraggingLooseWire(VertexId, Vec2),
    DraggingConnectedWire(VertexId, VertexId),
}

struct Cursor {
    position: Vec2,
    state: CursorState,
}

#[macroquad::main("GRAF")]
async fn main() {
    let mut cursor = CursorState::HoveringNothing;
    let mut session = Session::new();
    let mut context_menu_position: Option<Vec2> = None;

    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            match session.get_device_at(vec2(mx, my)) {
                Some(vid) => cursor = CursorState::DraggingDevice(vid),
                None => {}
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            cursor = CursorState::HoveringNothing;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let (mx, my) = mouse_position();
            context_menu_position = Some(vec2(mx, my));
        }

        // update
        if let CursorState::DraggingDevice(id) = cursor {
            let (mx, my) = mouse_position();
            session.move_device(id, vec2(mx, my));
        }

        if let Some(pos) = context_menu_position {
            Window::new(hash!(), pos, vec2(60., 90.))
                .label("New device")
                .movable(false)
                .ui(&mut *root_ui(), |ui| {
                    if ui.button(None, "Clock") {
                        session.create_clock(pos);
                        context_menu_position = None;
                    }
                    if ui.button(None, "Gate") {
                        session.create_gate(pos);
                        context_menu_position = None;
                    }
                    if ui.button(None, "Note") {
                        session.create_note(pos);
                        context_menu_position = None;
                    }
                });
        }

        clear_background(BLACK);
        session.draw();

        next_frame().await
    }
}
