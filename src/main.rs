use app::App;
use macroquad::prelude::*;

mod app;
mod dag;
mod devices;
mod drawing_utils;
mod session;

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
        app.handle_inputs();

        app.draw();

        next_frame().await
    }
}
