use std::path::Path;

use runty8::{
    runtime::{draw_context::DrawContext, state::State},
    App,
};

fn create_directory() -> String {
    // let buf = Path::new(file!()).with_extension("");
    let buf = Path::new("src/bin/ui_demo").to_path_buf();
    let dir_name = buf.to_str().unwrap();

    if let Err(e) = std::fs::create_dir(dir_name) {
        println!("Couldn't create directory, error: {:?}", e);
    };

    dir_name.to_owned()
}

fn main() {
    let assets_path = create_directory();

    runty8::run_app::<EmptyApp>(assets_path);
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
