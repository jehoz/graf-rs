use app::App;
use macroquad::prelude::*;

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
    let mut app = App::new();

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
