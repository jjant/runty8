use runty8::app::{App, Left, WhichOne};
use runty8::runtime::draw_context::DrawContext;
use runty8::runtime::state::{Button, State};

fn main() {
    runty8::run_app::<ExampleApp>("".to_owned()).unwrap();
}

pub struct ExampleApp {
    x: i32,
    y: i32,
    xa: i32,
    ya: i32,
    yc: i32,
}

impl WhichOne for ExampleApp {
    type Which = Left;
}

impl App for ExampleApp {
    fn init(_: &State) -> Self {
        Self {
            x: 6400,
            y: 6400,
            xa: 0,
            ya: 0,
            yc: 0,
        }
    }

    fn draw(&mut self, draw_context: &mut DrawContext) {
        draw_context.cls();
        draw_context.print(
            &format!("X={} Y={} YC={}", self.x / 100, self.y / 100, self.yc),
            0,
            0,
            6,
        );
        draw_context.line(0, 72, 127, 72, 4);

        let x0 = self.x / 100 + 1;
        let y0 = self.y / 100 + 1;

        #[allow(dead_code, unused_variables)]
        let sprite: [u8; 8 * 8] = [
            0, 0, 0, 0, 0, 0, 0, 0, //
            6, 6, 6, 6, 6, 6, 6, 0, //
            6, 7, 7, 7, 7, 7, 6, 0, //
            6, 7, 6, 6, 6, 7, 6, 0, //
            6, 7, 6, 6, 6, 7, 6, 0, //
            6, 7, 6, 6, 6, 7, 6, 0, //
            6, 7, 7, 7, 7, 7, 6, 0, //
            6, 6, 6, 6, 6, 6, 6, 0, //
        ];

        draw_context.rectfill(x0, y0, x0 + 6, y0 + 6, 4);
        // draw_context.raw_spr(sprite, self.x / 100, self.y / 100);
    }

    fn update(&mut self, state: &State) {
        // println!("{:?}", state);

        // -- lower -100/100 less max spd
        // -- lower -10/10 slower start
        // if (btn(⬅️) and xa>-100) xa-=10
        // if (btn(➡️) and xa<100) xa+=10

        // -- lower -200 is higher jump
        // if (btn(🅾️) and yc==0) ya=-200 yc=39

        // -- lower 5 is more slippery
        // -- note: must be divisible by
        // -- walking speed
        // if (xa<0) xa+=5
        // if (xa>0) xa-=5
        // x+=xa

        // -- lower -1 is more gravity
        // -- note: must be divisible by
        // -- jumping strength
        // if (yc>0) ya+=10 y+=ya yc-=1

        if state.btn(Button::Left) && self.xa > -100 {
            self.xa -= 10;
        }
        if state.btn(Button::Right) && self.xa < 100 {
            self.xa += 10;
        }

        if state.btn(Button::C) && self.yc == 0 {
            self.ya = -200;
            self.yc = 39;
        }

        self.xa += match self.xa.cmp(&0) {
            std::cmp::Ordering::Less => 5,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -5,
        };

        self.x += self.xa;

        if self.yc > 0 {
            self.ya += 10;
            self.y += self.ya;
            self.yc -= 1;
        }
    }
}
