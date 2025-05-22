use devices::{ClockDevice, Device, GateDevice, NoteDevice};
use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
};
use session::Session;

mod dag;
mod devices;
mod session;

const NODE_SIZE: f32 = 50.0;
const HALF_NODE_SIZE: f32 = NODE_SIZE / 2.0;

#[macroquad::main("GRAF")]
async fn main() {
    let mut session = Session::new();
    let mut context_menu_position: Option<Vec2> = None;

    loop {
        clear_background(BLACK);

        let (mouse_x, mouse_y) = mouse_position();
        draw_text(
            &format!("Mouse: ({mouse_x}, {mouse_y})"),
            20.0,
            20.0,
            30.0,
            WHITE,
        );

        for (_node_id, node) in session.nodes.iter() {
            draw_rectangle_lines(
                node.position.x - HALF_NODE_SIZE,
                node.position.y - HALF_NODE_SIZE,
                NODE_SIZE,
                NODE_SIZE,
                2.0,
                WHITE,
            );
        }

        if is_mouse_button_released(MouseButton::Right) {
            let (mx, my) = mouse_position();
            context_menu_position = Some(vec2(mx, my));
            // nodes.insert(
            //     NodeId(node_id_counter),
            //     CircuitNode {
            //         pos: Vec2::new(nx, ny),
            //     },
            // );
            // node_id_counter += 1;
        }

        if let Some(pos) = context_menu_position {
            root_ui().window(hash!(), pos, vec2(50., 90.), |ui| {
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

        next_frame().await
    }
}
