use std::path::Path;

use runty8::{
    runtime::{draw_context::DrawContext, state::State},
    App,
};

fn assets_path() -> String {
    let buf = Path::new("src/bin/ui_demo").to_path_buf();
    let dir_name = buf.to_str().unwrap();
    dir_name.to_owned()
}

fn main() {
    runty8::run_app::<EmptyApp>(assets_path());
}

struct EmptyApp;

impl App for EmptyApp {
    fn init() -> Self {
        Self
    }

    fn update(&mut self, _: &State) {}

    fn draw(&self, draw: &mut DrawContext) {
        draw.cls();
        draw.print("EMPTY", 0, 0, 7);
    }
}
