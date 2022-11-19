use rand::Rng;
use runty8_runtime::{App, Flags, Map, Pico8, Resources, SpriteSheet};

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

struct Game;

impl App for Game {
    fn init(pico8: &mut Pico8) -> Self {
        log::debug!("Game::init called");
        pico8.rect(15, 15, 30, 30, 8);
        pico8.set_title("nice".to_string());
        Self
    }

    fn update(&mut self, pico8: &mut Pico8) {
        log::debug!("Game::update called");
        println!("Game::update called");
        //   pico8.cls(0);
        pico8.print("SOMETHING", 8, 8, 7);
    }

    fn draw(&mut self, pico8: &mut Pico8) {
        log::debug!("Game::draw called");
        println!("Game::draw called");
        pico8.rect(
            rand::thread_rng().gen_range(0..128),
            rand::thread_rng().gen_range(0..128),
            rand::thread_rng().gen_range(0..128),
            rand::thread_rng().gen_range(0..128),
            8,
        );
    }
}
