use rand::Rng;
use runty8_runtime::{App, Button, Flags, Map, Pico8, Resources, SpriteSheet};

const LOG: bool = false;

fn main() {
    unsafe {
        runty8_event_loop::event_loop::<Game>(Resources {
            assets_path: "standalone-game/assets".to_owned(),
            map: Map::new(),
            sprite_flags: Flags::new(),
            sprite_sheet: SpriteSheet::new(),
        });
    }
}

struct Game {
    x: i32,
    y: i32,
    mouse_x: i32,
    mouse_y: i32,
    mouse: bool,
}

impl Game {
    const W: i32 = 3;
    const H: i32 = 3;
}

impl App for Game {
    fn init(pico8: &mut Pico8) -> Self {
        log::debug!("Game::init called");
        pico8.rect(15, 15, 30, 30, 8);
        pico8.set_title("nice".to_string());

        let (mouse_x, mouse_y) = pico8.mouse();

        Self {
            x: 64,
            y: 64,
            mouse_x,
            mouse_y,
            mouse: pico8.btn(Button::Mouse),
        }
    }

    fn update(&mut self, pico8: &mut Pico8) {
        if LOG {
            log::debug!("Game::update called");
            println!("Game::update called");
        }

        if pico8.btn(Button::Down) {
            self.y += 1
        } else if pico8.btn(Button::Up) {
            self.y -= 1
        }
        if pico8.btn(Button::Right) {
            self.x += 1
        } else if pico8.btn(Button::Left) {
            self.x -= 1
        }
        self.x = clamp(0, self.x, 128 - Self::W);
        self.y = clamp(0, self.y, 128 - Self::H);

        (self.mouse_x, self.mouse_y) = pico8.mouse();
        self.mouse = pico8.btn(Button::Mouse);
    }

    fn draw(&mut self, pico8: &mut Pico8) {
        if LOG {
            log::debug!("Game::draw called");
            println!("Game::draw called");
        }
        pico8.cls(0);

        for i in 0..16 {
            let x = i % 4;
            let y = i / 4;

            pico8.rectfill(x * 4, y * 4, x * 4 + 3, y * 4 + 3, i as u8);
        }
        pico8.print("USE ARROW KEYS TO MOVE", 8, 8, 12);
        pico8.rectfill(
            self.x,
            self.y,
            self.x + (Self::W - 1),
            self.y + (Self::H - 1),
            7,
        );

        pico8.pset(self.mouse_x, self.mouse_y, if self.mouse { 9 } else { 12 });
    }
}

fn clamp(min: i32, v: i32, max: i32) -> i32 {
    runty8_runtime::mid(min as f32, v as f32, max as f32) as i32
}
