use runty8::{self, App, Button, Pico8};

fn main() {
    runty8::run_app::<Sprites>("examples/sprites".to_owned()).unwrap();
}

struct Sprites {}
impl Sprites {
    fn do_draw(&self, pico8: &mut Pico8) {
        pico8.cls(0);

        for i in 0..(128 / 8) {
            pico8.line(0, i * 8, 127, i * 8, if i % 2 == 0 { 7 } else { 5 });
            pico8.line(i * 8, 0, i * 8, 127, if i % 2 == 0 { 6 } else { 13 });
        }

        pico8.spr_(32, 32, 32, 0.5, 0.5, false, false);
        pico8.spr_(32, 40, 40, 1.0, 1.0, false, false);

        // TEST FLIPS
        const RIGHT_ARROW: usize = 1;
        const UP_ARROW: usize = 2;
        const UP_RIGHT_ARROW: usize = 3;

        pico8.spr_(RIGHT_ARROW, 48, 48, 1.0, 1.0, false, false);
        pico8.spr_(RIGHT_ARROW, 48, 64, 1.0, 1.0, true, false);
        pico8.spr_(UP_ARROW, 48, 80, 1.0, 1.0, false, false);
        pico8.spr_(UP_ARROW, 48, 96, 1.0, 1.0, false, true);

        pico8.spr_(UP_RIGHT_ARROW, 80, 48, 1.0, 1.0, false, false);
        pico8.spr_(UP_RIGHT_ARROW, 80, 64, 1.0, 1.0, true, false);
        pico8.spr_(UP_RIGHT_ARROW, 80, 80, 1.0, 1.0, false, true);
        pico8.spr_(UP_RIGHT_ARROW, 80, 96, 1.0, 1.0, true, true);

        // TEST BIG SPRITES
        pico8.print("BIG SPRITE", 16, 8, 7);
        pico8.spr_(7, 16, 16, 2.0, 2.0, false, false);
    }
}

impl App for Sprites {
    fn init(pico8: &mut Pico8) -> Self {
        let this = Self {};

        this.do_draw(pico8);
        this
    }

    fn update(&mut self, pico8: &mut Pico8) {
        if pico8.btnp(Button::C) {
            self.do_draw(pico8);
        }
    }

    fn draw(&mut self, _: &mut Pico8) {}
}
