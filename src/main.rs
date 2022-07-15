use std::path::Path;

use runty8::App;
use runty8::DrawContext;

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
    fn init(_: &mut DrawContext) -> Self {
        Self
    }

    fn update(&mut self, _: &mut DrawContext) {}

    fn draw(&mut self, draw: &mut DrawContext) {
        draw.cls();
        draw.print("EMPTY", 0, 0, 7);
    }
}
