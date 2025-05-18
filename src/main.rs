use std::collections::{HashMap, HashSet};

use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
};

#[derive(Eq, Hash, PartialEq)]
struct NodeId(u32);

struct CircuitNode {
    pos: Vec2,
}

enum Device {
    Pulsar(Pulsar),
    Gate(Gate),
}

struct Pulsar {
    frequency: f32,
    duty_cycle: f32,
}

struct Gate {
    inputs: HashSet<NodeId>,
    // glyph: Glyph,
}

const NODE_SIZE: f32 = 50.0;
const HALF_NODE_SIZE: f32 = NODE_SIZE / 2.0;

#[macroquad::main("GRAF")]
async fn main() {
    let mut node_id_counter: u32 = 0;
    let mut nodes: HashMap<NodeId, CircuitNode> = HashMap::new();

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

        for (_node_id, node) in nodes.iter() {
            draw_rectangle_lines(
                node.pos.x - HALF_NODE_SIZE,
                node.pos.y - HALF_NODE_SIZE,
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
                    context_menu_position = None;
                }
                if ui.button(None, "Gate") {
                    context_menu_position = None;
                }
                if ui.button(None, "Note") {
                    context_menu_position = None;
                }
            });
        }

        next_frame().await
    }
}
