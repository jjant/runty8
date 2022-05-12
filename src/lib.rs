#![allow(clippy::new_without_default)]
pub mod app;
mod draw;
pub mod editor;
mod font;
pub mod graphics;
pub mod runtime;
pub mod screen;
pub mod ui;
pub use app::App;
use runtime::{
    draw_context::DrawData,
    map::Map,
    sprite_sheet::{Color, Sprite, SpriteSheet},
    state::{Flags, State},
};

#[derive(Clone, Copy)]
pub enum MouseButton {
    // TODO: Handle other mouse buttons? idk
    Left,
    Right,
    Middle,
}

#[derive(Clone, Copy)]
pub enum MouseEvent {
    // Current position of the mouse
    // NOT delta
    Move { x: i32, y: i32 },
    Down(MouseButton),
    Up(MouseButton),
}

#[derive(Clone, Copy)]
pub enum Event {
    Mouse(MouseEvent),
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

pub fn run_app<T: App + 'static>(assets_path: String) {
    let map: Map = Map::new();
    let sprite_flags: Flags = create_sprite_flags(&assets_path);
    let sprite_sheet = create_sprite_sheet(&assets_path);

    let draw_data = DrawData::new();

    crate::screen::run_app::<T>(assets_path, map, sprite_flags, sprite_sheet, draw_data);
}
