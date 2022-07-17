use std::path::Path;

use runty8::{App, Pico8};

fn assets_path() -> String {
    let buf = Path::new("src/bin/ui_demo").to_path_buf();
    let dir_name = buf.to_str().unwrap();
    dir_name.to_owned()
}

fn main() {
    runty8::run_app::<EmptyApp>(assets_path()).unwrap()
}

struct EmptyApp;

impl App for EmptyApp {
    fn init(_: &mut dyn Pico8) -> Self {
        Self
    }

    fn update(&mut self, _: &mut dyn Pico8) {}

    fn draw(&mut self, draw: &mut dyn Pico8) {
        draw.cls(0);
        draw.print("EMPTY", 0, 0, 7);
    }
}
