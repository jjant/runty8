use runty8::{App, Pico8};
// use runty8_graphics;

fn main() {
    runty8::run_app::<EmptyApp>("src/editor_assets".to_owned()).unwrap();
}

struct EmptyApp;

impl App for EmptyApp {
    fn init(_: &mut Pico8) -> Self {
        Self
    }

    fn update(&mut self, _: &mut Pico8) {}

    fn draw(&mut self, draw: &mut Pico8) {
        draw.cls(0);
        draw.print("EMPTY", 0, 0, 7);
    }
}
