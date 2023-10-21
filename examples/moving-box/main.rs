use runty8::{App, Button, Pico8};
use runty8_core::SpriteSheet;

fn main() {
    let cartridge = std::fs::read_to_string("./examples/moving-box/big_cartridge.p8").unwrap();
    let sprite_sheet = process_cartridge(cartridge);
    println!("{sprite_sheet:#?}");

    let mut resources = runty8::load_assets!("moving-box").unwrap();
    resources.sprite_sheet = sprite_sheet;
    runty8::debug_run::<ExampleApp>(resources).unwrap();
}

pub struct ExampleApp {
    x: i32,
    y: i32,
    xa: i32,
    ya: i32,
    yc: i32,
}

impl App for ExampleApp {
    fn init(_: &mut Pico8) -> Self {
        Self {
            x: 6400,
            y: 6400,
            xa: 0,
            ya: 0,
            yc: 0,
        }
    }

    fn draw(&mut self, draw_context: &mut Pico8) {
        draw_context.cls(0);
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

    fn update(&mut self, state: &mut Pico8) {
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

        if state.btn(Button::Circle) && self.yc == 0 {
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

fn process_cartridge(cartridge_content: String) -> SpriteSheet {
    const GFX: &str = "__gfx__\n";
    // Get gfx data
    let start_byte = cartridge_content.find(GFX).unwrap();
    // TODO: label section seems to be optional
    let end_byte = cartridge_content.find("__label__").unwrap();

    let gfx_data = &cartridge_content[(start_byte + GFX.len())..end_byte];

    let mut sprite_sheet = SpriteSheet::new();

    for (y, line) in gfx_data.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let color = char.to_digit(16).unwrap() as u8;

            sprite_sheet.set(x, y, color);
        }
    }
    return sprite_sheet;
    // return SpriteSheet::deserialize(gfx_data).unwrap();
}

// Cartridge format:
// pico-8 cartridge // http://www.pico-8.com
// version 41
// __lua__
// [LUA CODE]
// __gfx__
// [SPRITE DATA?]
// __label__
// [LABEL DATA? (cartridge cover I guess)]
// __gff__
// [Not sure what this is]
// __map__
// [MAP DATA]
// __sfx__
// [SOUND DATA]
// __music__
// [MUSIC DATA]
// [EMPTY LINE]
