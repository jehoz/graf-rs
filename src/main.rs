use app::App;
use egui::CornerRadius;
use macroquad::prelude::*;

use crate::drawing_utils::ColorPalette;

mod app;
mod dag;
mod devices;
mod drawing_utils;
mod midi;
mod session;
mod widgets;

fn window_conf() -> Conf {
    Conf {
        window_title: "GRAF".to_owned(),
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let colors = ColorPalette {
        fg_0: Color::from_hex(0xFFFFFF),
        fg_1: Color::from_hex(0xF4F2F5),
        fg_2: Color::from_hex(0xE8E4EB),
        fg_3: Color::from_hex(0xD7D0DB),

        bg_0: Color::from_hex(0x201B24),
        bg_1: Color::from_hex(0x332B38),
        bg_2: Color::from_hex(0x453B4D),
        bg_3: Color::from_hex(0x53475C),

        error: Color::from_hex(0xF21B1B),
    };

    let mut app = App::new(colors);

    loop {
        let mut egui_wants_pointer = false;
        egui_macroquad::ui(|ctx| {
            app.ui(ctx);
            if ctx.wants_pointer_input() {
                egui_wants_pointer = true;
            }
        });

        if !egui_wants_pointer {
            app.handle_inputs();
        }
        app.update();
        app.draw();
        egui_macroquad::draw();

        next_frame().await
    }
}
