#![allow(clippy::new_without_default)]
mod draw;
pub mod editor;
mod font;
pub mod graphics;
pub mod runtime;
pub mod screen;
pub mod ui;
use runtime::{
    draw_context::{DrawContext, DrawData},
    map::Map,
    sprite_sheet::{Color, Sprite, SpriteSheet},
    state::{Flags, State},
};

/// A regular pico8 app
pub trait App {
    fn init() -> Self;
    fn update(&mut self, state: &State);
    fn draw(&self, draw_context: &mut DrawContext);
}

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

fn create_map(assets_path: &str) -> Map {
    // let path = format!(
    //     "{}{}{}",
    //     assets_path,
    //     std::path::MAIN_SEPARATOR,
    //     Map::file_name()
    // );

    // if let Ok(content) = std::fs::read_to_string(&path) {
    //     Map::deserialize(&content).unwrap()
    // } else {
    //     println!("Couldn't read spreadsheet from {}", path);
    //     Map::new()
    // }
    Map::new()
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

fn create_directory(path: &str) {
    if let Err(e) = std::fs::create_dir(path) {
        println!("Couldn't create directory {}, error: {:?}", path, e);
    };
}

pub fn run_app<T: App + 'static>(assets_path: String) {
    create_directory(&assets_path);

    let map: Map = create_map(&assets_path);
    let sprite_flags: Flags = create_sprite_flags(&assets_path);
    let sprite_sheet = create_sprite_sheet(&assets_path);

    let draw_data = DrawData::new();

    println!("{}", map.serialize());
    crate::screen::run_app::<T>(assets_path, map, sprite_flags, sprite_sheet, draw_data);
}
