use std::path::{Path, PathBuf};

use runty8::runtime::{
    draw_context::DrawData,
    map::Map,
    sprite_sheet::SpriteSheet,
    state::{Flags, State},
};

fn create_directory() -> &'static str {
    let buf = Path::new(file!()).with_extension("");
    let buf = Path::new("src/bin/ui_demo").to_path_buf();
    let dir_name = buf.to_str().unwrap();

    if let Err(e) = std::fs::create_dir(dir_name) {
        println!("Couldn't create directory, error: {:?}", e);
    };

    Box::leak(Box::from(dir_name))
}

fn create_sprite_flags(assets_path: &str) -> Flags {
    if let Ok(content) = std::fs::read_to_string(&format!(
        "{}{}{}",
        assets_path,
        std::path::MAIN_SEPARATOR,
        Flags::file_name()
    )) {
        Flags::deserialize(&content).unwrap()
    } else {
        Flags::new()
    }
}

fn create_sprite_sheet(assets_path: &str) -> SpriteSheet {
    let path = format!(
        "{}{}{}",
        assets_path,
        std::path::MAIN_SEPARATOR,
        SpriteSheet::file_name()
    );

    if let Ok(content) = std::fs::read_to_string(&path) {
        SpriteSheet::deserialize(&content).unwrap()
    } else {
        println!("Couldn't read spreadsheet from {}", path);
        SpriteSheet::new()
    }
}

fn main() {
    let assets_path = create_directory();

    let map: &'static Map = Box::leak(Box::new(Map::new()));
    let sprite_flags: &'static Flags = Box::leak(Box::new(create_sprite_flags(assets_path)));
    let sprite_sheet = create_sprite_sheet(assets_path);

    let state = State::new(assets_path, sprite_sheet, sprite_flags, map);
    let draw_data = DrawData::new();

    runty8::screen::run_app((map, sprite_flags), state, draw_data);
}
