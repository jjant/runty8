use runty8::{App, Pico8};

/// A "game" to edit the editor assets themselves
fn main() {
    let resources = runty8::load_assets!("../src/runty8-editor/src/editor_assets").unwrap();

    runty8::debug_run::<Confetti>(resources).unwrap();
}

struct Confetti;

impl App for Confetti {
    fn init(_pico8: &mut Pico8) -> Self {
        Self
    }

    fn update(&mut self, _pico8: &mut Pico8) {}

    fn draw(&mut self, pico8: &mut Pico8) {
        pico8.cls(0);

        for i in 0..16 {
            for j in 0..16 {
                pico8.spr(i + j * 16, i as i32 * 8, j as i32 * 8);
            }
        }
    }
}
