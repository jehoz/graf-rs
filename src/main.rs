use std::collections::HashMap;

use macroquad::prelude::*;

#[derive(Eq, Hash, PartialEq)]
struct NodeId(u32);

struct CircuitNode {
    pos: Vec2,
}

const NODE_SIZE: f32 = 50.0;
const HALF_NODE_SIZE: f32 = NODE_SIZE / 2.0;

#[macroquad::main("GRAF")]
async fn main() {
    let mut node_id_counter: u32 = 0;
    let mut nodes: HashMap<NodeId, CircuitNode> = HashMap::new();
    // let mut cursor:

    loop {
        clear_background(BLACK);

        draw_text("G R A f", 20.0, 20.0, 30.0, WHITE);

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
            let (nx, ny) = mouse_position();
            nodes.insert(
                NodeId(node_id_counter),
                CircuitNode {
                    pos: Vec2::new(nx, ny),
                },
            );
            node_id_counter += 1;
        }

        next_frame().await
    }
}
