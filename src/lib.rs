#![allow(clippy::new_without_default)]
mod draw;
pub mod editor;
mod font;
pub mod graphics;
pub mod runtime;
pub mod screen;
pub mod ui;
use crate::editor::serialize::Serialize;
use glium::glutin::event::VirtualKeyCode;
use runtime::{
    draw_context::{DrawContext, DrawData},
    flags::Flags,
    map::Map,
    sprite_sheet::{Color, Sprite, SpriteSheet},
    state::State,
};

/// A regular pico8 app
pub trait App {
    fn init() -> Self;
    fn update(&mut self, state: &State);
    fn draw(&self, draw_context: &mut DrawContext);
}

#[derive(Clone, Copy, Debug)]
pub enum MouseButton {
    // TODO: Handle other mouse buttons? idk
    Left,
    Right,
    Middle,
}

#[derive(Clone, Copy, Debug)]
pub enum MouseEvent {
    // Current position of the mouse
    // NOT delta
    Move { x: i32, y: i32 },
    Down(MouseButton),
    Up(MouseButton),
}

#[derive(Clone, Copy, Debug)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Key {
    pub(crate) fn from_virtual_keycode(key: VirtualKeyCode) -> Option<Self> {
        match key {
            VirtualKeyCode::A => Some(Self::A),
            VirtualKeyCode::B => Some(Self::B),
            VirtualKeyCode::C => Some(Self::C),
            VirtualKeyCode::D => Some(Self::D),
            VirtualKeyCode::E => Some(Self::E),
            VirtualKeyCode::F => Some(Self::F),
            VirtualKeyCode::G => Some(Self::G),
            VirtualKeyCode::H => Some(Self::H),
            VirtualKeyCode::I => Some(Self::I),
            VirtualKeyCode::J => Some(Self::J),
            VirtualKeyCode::K => Some(Self::K),
            VirtualKeyCode::L => Some(Self::L),
            VirtualKeyCode::M => Some(Self::M),
            VirtualKeyCode::N => Some(Self::N),
            VirtualKeyCode::O => Some(Self::O),
            VirtualKeyCode::P => Some(Self::P),
            VirtualKeyCode::Q => Some(Self::Q),
            VirtualKeyCode::R => Some(Self::R),
            VirtualKeyCode::S => Some(Self::S),
            VirtualKeyCode::T => Some(Self::T),
            VirtualKeyCode::U => Some(Self::U),
            VirtualKeyCode::V => Some(Self::V),
            VirtualKeyCode::W => Some(Self::W),
            VirtualKeyCode::X => Some(Self::X),
            VirtualKeyCode::Y => Some(Self::Y),
            VirtualKeyCode::Z => Some(Self::Z),
            // VirtualKeyCode::Escape => todo!(),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum KeyboardEvent {
    Up(Key),
    Down(Key),
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
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
    let path = format!(
        "{}{}{}",
        assets_path,
        std::path::MAIN_SEPARATOR,
        Map::file_name()
    );

    if let Ok(content) = std::fs::read_to_string(&path) {
        Map::deserialize(&content).unwrap()
    } else {
        println!("Couldn't read map from {}", path);
        Map::new()
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
        println!("Couldn't read sprite sheet from {}", path);
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

    crate::screen::run_app::<T>(assets_path, map, sprite_flags, sprite_sheet, draw_data);
}

/* UTILS */
pub fn write_and_log(file_name: &str, contents: &str) {
    print!("Writing {file_name}... ");
    std::fs::write(&file_name, contents).unwrap();
    println!("success.")
}
