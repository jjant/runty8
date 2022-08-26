use rand::Rng;
use runty8::{self, App, Button, Pico8};

fn main() {
    runty8::run_app::<Circles>("examples/circles".to_owned()).unwrap();
}

struct Circles {}
impl Circles {
    fn do_draw(&self, pico8: &mut Pico8) {
        pico8.cls(0);

        pico8.rect(0, 0, 127, 127, 7);

        fn rand(n: i32) -> i32 {
            rand::thread_rng().gen_range(0..n)
        }

        for _ in 0..=7 {
            pico8.rectfill(
                rand(128),
                rand(128),
                rand(128),
                rand(128),
                rand::thread_rng().gen_range(9..16),
            );
            pico8.rect(
                rand(128),
                rand(128),
                rand(128),
                rand(128),
                rand::thread_rng().gen_range(1..8),
            );
        }
    }
}

impl App for Circles {
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
