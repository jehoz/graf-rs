use macroquad::prelude::*;

struct CircuitNode {
    pos: Vec2,
}

const NODE_SIZE: f32 = 50.0;
const HALF_NODE_SIZE: f32 = NODE_SIZE / 2.0;

#[macroquad::main("GRAF")]
async fn main() {
    let mut nodes: Vec<CircuitNode> = Vec::new();

    loop {
        clear_background(BLACK);

        draw_text("G R A f", 20.0, 20.0, 30.0, WHITE);

        for node in nodes.iter() {
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
            nodes.push(CircuitNode {
                pos: Vec2::new(nx, ny),
            })
        }

        next_frame().await
    }
}
