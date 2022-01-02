use runty8::{app::App, Button, DrawContext, State};
mod examples;

fn main() {
    runty8::run_editor();

    // runty8::run_app::<ExampleApp>();
}

pub struct ExampleApp {
    x: i32,
    y: i32,
    xa: i32,
    ya: i32,
    yc: i32,
}

impl App for ExampleApp {
    fn init() -> Self {
        Self {
            x: 6400,
            y: 6400,
            xa: 0,
            ya: 0,
            yc: 0,
        }
    }

    fn draw(&self, draw_context: &mut DrawContext) {
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

        if self.xa < 0 {
            self.xa += 5;
        } else if self.xa > 0 {
            self.xa -= 5;
        }
        self.x += self.xa;

        if self.yc > 0 {
            self.ya += 10;
            self.y += self.ya;
            self.yc -= 1;
        }
    }
}

// _set_fps(60)
// x=6000 -- player x-position
// y=6400 -- player y-position
// xa=0 -- x-acceleration
// ya=0 -- y-acceleration
// yc=0 -- y jump count

// repeat

// cls()
// print("x="..flr(x/100).." y="..flr(y/100).." yc="..yc,0,0,6)
// line(0,72,127,72,4)
// spr(1,x/100,y/100)
// flip()

// until forever

// fn set_color(buf: &mut [u8], x: usize, y: usize, color: [u8; NUM_COMPONENTS]) {
//     for i in 0..NUM_COMPONENTS {
//         buf[NUM_COMPONENTS * (x + y * WIDTH) + i] = color[i];
//     }
// }

// fn main() {
//     // let buffer = [[0_u32; 128]; 128];
//     let mut buffer: [u8; NUM_COMPONENTS * WIDTH * WIDTH] = [30_u8; NUM_COMPONENTS * WIDTH * WIDTH];

//     set_color(&mut buffer, 50, 50, [255, 0, 255]);
//     set_color(&mut buffer, 51, 51, [255, 0, 255]);
//     set_color(&mut buffer, 52, 52, [255, 0, 255]);
//     set_color(&mut buffer, 53, 53, [255, 0, 255]);

//     set_color(&mut buffer, 0, 0, [0, 0, 0]);

//     set_color(&mut buffer, 127, 0, [0, 0, 255]);
//     set_color(&mut buffer, 0, 127, [0, 255, 0]);

//     set_color(&mut buffer, 127, 127, [255, 255, 255]);

//     let a = 5;
//     screen::do_something(buffer.to_vec(), |logical_position, buffer| {
//         let (x, y) = (
//             logical_position.x / (600. / 128.),
//             logical_position.y / (600. / 128.),
//         );
//         println!("{:?}", (x, y));

//         set_color(
//             buffer,
//             x.floor() as usize,
//             y.floor() as usize,
//             [255, 255, 255],
//         );
//     });
// }
