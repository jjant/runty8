use runty8::{app::App, Button, Color, DrawContext, State};
mod examples;

fn main() {
    runty8::run_app::<SpriteEditor>();

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
        // println!("x={} y={} yc={}", self.x / 100, self.y / 100, self.yc);
        // draw_context.print(
        //     format!("x={} y={} yc={}", self.x / 100, self.y / 100, self.yc),
        //     0,
        //     0,
        //     6,
        // );
        draw_context.line(0, 72, 127, 72, 4);

        let x0 = self.x / 100 + 1;
        let y0 = self.y / 100 + 1;

        #[rustfmt::skip]
        let sprite: [u8; 8 * 8] = [
            0, 0, 0, 0, 0, 0, 0, 0,
            6, 6, 6, 6, 6, 6, 6, 0,
            6, 7, 7, 7, 7, 7, 6, 0,
            6, 7, 6, 6, 6, 7, 6, 0,
            6, 7, 6, 6, 6, 7, 6, 0,
            6, 7, 6, 6, 6, 7, 6, 0,
            6, 7, 7, 7, 7, 7, 6, 0,
            6, 6, 6, 6, 6, 6, 6, 0,
        ];

        draw_context.rectfill(x0, y0, x0 + 6, y0 + 6, 4);
        draw_context.raw_spr(sprite, self.x / 100, self.y / 100);
    }

    fn update(&mut self, state: &State) {
        println!("{:?}", state);

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

struct SpriteEditor {
    mouse_x: i32,
    mouse_y: i32,
    mouse_pressed: bool,
    highlighted_color: Color,
    sprite_sheet: Vec<Color>,
    bottom_text: String,
}

const CANVAS_X: i32 = 79; // end = 120
const CANVAS_Y: i32 = 10;

impl App for SpriteEditor {
    fn init() -> Self {
        Self {
            mouse_x: 64,
            mouse_y: 64,
            mouse_pressed: false,
            highlighted_color: 11,
            sprite_sheet: vec![0; 8 * 8],
            bottom_text: String::new(),
        }
    }

    fn update(&mut self, state: &State) {
        println!("{:?}", state);
        self.mouse_x = state.mouse_x;
        self.mouse_y = state.mouse_y;
        self.mouse_pressed = state.mouse_pressed;
        self.bottom_text = String::new();

        for color in 0..16 {
            if color_position(color).contains(self.mouse_x, self.mouse_y) {
                self.bottom_text = format!("COLOUR{}", color);
                if self.mouse_pressed {
                    self.highlighted_color = color;
                }
                return;
            }
        }

        if canvas_position().contains(self.mouse_x, self.mouse_y) {
            for x in 0..8 {
                for y in 0..8 {
                    if canvas_pixel_rect(x, y).contains(self.mouse_x, self.mouse_y) {
                        self.bottom_text = format!("X:{}@Y:{}", x, y);
                        if self.mouse_pressed {
                            self.sprite_sheet[(x + y * 8) as usize] = self.highlighted_color;
                        }
                    }
                }
            }
        }
    }

    fn draw(&self, draw_context: &mut DrawContext) {
        const MOUSE_SPRITE: [u8; 8 * 8] = [
            0, 1, 0, 0, 0, 0, 0, 0, //
            1, 7, 1, 0, 0, 0, 0, 0, //
            1, 7, 7, 1, 0, 0, 0, 0, //
            1, 7, 7, 7, 1, 0, 0, 0, //
            1, 7, 7, 7, 7, 1, 0, 0, //
            1, 7, 7, 1, 1, 0, 0, 0, //
            0, 1, 1, 7, 1, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
        ];

        draw_context.cls();
        draw_context.rectfill(0, 0, 127, 127, 5);

        // Draw menu bars
        draw_context.rectfill(0, 0, 127, 7, 8);
        draw_context.rectfill(0, 121, 127, 127, 8);

        // draw canvas
        canvas_position().fill(draw_context, 0);
        for x in 0..8 {
            for y in 0..8 {
                canvas_pixel_rect(x, y).fill(draw_context, self.sprite_sheet[(x + y * 8) as usize])
            }
        }

        // Draw color palette

        draw_context.rectfill(
            CANVAS_X,
            CANVAS_Y,
            CANVAS_X + BOX_SIZE - 1,
            CANVAS_Y + BOX_SIZE - 1,
            0,
        );

        for color in 0..16 {
            let Rect {
                x,
                y,
                width,
                height,
            } = color_position(color);

            draw_context.rectfill(x, y, x + width - 1, y + height - 1, color as u8);
        }

        // draw highlight
        let Rect {
            x,
            y,
            width,
            height,
        } = color_position(self.highlighted_color);
        draw_context.rect(x, y, x + width - 1, y + height - 1, 0);
        draw_context.rect(x - 1, y - 1, x + width, y + height, 7);

        draw_context.print(&self.bottom_text, 1, 122, 2);

        // Always render the mouse last (on top of everything)
        draw_context.raw_spr(MOUSE_SPRITE, self.mouse_x, self.mouse_y);
    }
}

const CANVAS_BORDER: i32 = 1;
fn canvas_position() -> Rect {
    Rect {
        x: 8,
        y: 10,
        width: 8 * 8 + CANVAS_BORDER * 2,
        height: 8 * 8 + CANVAS_BORDER * 2,
    }
}

fn canvas_pixel_rect(x: i32, y: i32) -> Rect {
    let canvas = canvas_position();

    Rect {
        x: canvas.x + CANVAS_BORDER + 8 * x,
        y: canvas.y + CANVAS_BORDER + 8 * y,
        width: 8,
        height: 8,
    }
}
const SIZE: i32 = 10;
const BOX_SIZE: i32 = 4 * SIZE + 2;

struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    pub fn contains(&self, x: i32, y: i32) -> bool {
        let contains_x = x >= self.x && x < self.x + self.width;
        let contains_y = y >= self.y && y < self.y + self.height;

        contains_x && contains_y
    }

    pub fn fill(&self, draw_context: &mut DrawContext, color: Color) {
        draw_context.rectfill(
            self.x,
            self.y,
            self.x + self.width - 1,
            self.y + self.height - 1,
            color,
        )
    }
}

fn color_position(color: Color) -> Rect {
    let x = CANVAS_X + 1 + (color as i32 % 4) * SIZE;
    let y = CANVAS_Y + 1 + (color as i32 / 4) * SIZE;

    Rect {
        x,
        y,
        width: SIZE,
        height: SIZE,
    }
}

//// DEBUG STUFF

#[allow(dead_code)]
fn print_debug_strings(draw_context: &mut DrawContext, x: i32, y: i32) {
    draw_context.print("0123456789:;<=>?@", x, y, 7);
    let mut letters = "abcdefghijklmnopqrstuvwxyz".to_owned();
    letters.make_ascii_uppercase();
    draw_context.print(&letters, x, y + 10, 7);
}
