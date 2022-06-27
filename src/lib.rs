#![doc = include_str!("../README.md")]
#![allow(clippy::new_without_default)]
// #![deny(missing_docs)]
pub mod app;
pub mod runtime;
pub mod ui;

mod controller;
mod draw;
mod editor;
mod font;
mod graphics;
mod run;
// mod screen;
use crate::editor::serialize::Serialize;
use app::AppCompat;
use glium::glutin::event::{ElementState, VirtualKeyCode};
use runtime::{
    draw_context::DrawData,
    flags::Flags,
    map::Map,
    sprite_sheet::{Color, Sprite, SpriteSheet},
};
use std::{f32::consts::PI, fmt::Debug};

/// Mouse buttons
#[derive(Clone, Copy, Debug)]
pub enum MouseButton {
    // TODO: Handle other mouse buttons? idk
    Left,
    Right,
    Middle,
}

/// Mouse-related events
#[derive(Clone, Copy, Debug)]
pub enum MouseEvent {
    /// Mouse move event.
    // Contains the current position of the mouse.
    Move {
        ///
        x: i32,
        ///
        y: i32,
    },
    // TODO: Refactor these two below to factor out the MouseButton
    /// Mouse button pressed
    Down(MouseButton),
    /// Mouse button released
    Up(MouseButton),
}

/// Keyboard keys
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
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
    Control,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    Escape,
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
            VirtualKeyCode::LControl => Some(Self::Control),
            VirtualKeyCode::Left => Some(Self::LeftArrow),
            VirtualKeyCode::Right => Some(Self::RightArrow),
            VirtualKeyCode::Up => Some(Self::UpArrow),
            VirtualKeyCode::Down => Some(Self::DownArrow),
            VirtualKeyCode::Escape => Some(Self::Escape),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct KeyboardEvent {
    pub key: Key,
    pub state: KeyState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyState {
    Up,
    Down,
}

impl KeyState {
    fn from_state(state: ElementState) -> Self {
        match state {
            ElementState::Pressed => Self::Down,
            ElementState::Released => Self::Up,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    Tick { delta_millis: f64 },
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

fn create_directory(path: &str) -> std::io::Result<()> {
    if let Err(e) = std::fs::create_dir(path) {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => {
                // This directory already existing is not really an error.
                Ok(())
            }
            _ => {
                eprintln!("Couldn't create assets directory: `{path}`.");

                Err(e)
            }
        }
    } else {
        Ok(())
    }
}

/// Run a Pico8 application
// TODO: add example
pub fn run_app<T: AppCompat + 'static>(assets_path: String) -> std::io::Result<()> {
    create_directory(&assets_path)?;

    let map: Map = create_map(&assets_path);
    let sprite_flags: Flags = create_sprite_flags(&assets_path);
    let sprite_sheet = create_sprite_sheet(&assets_path);

    let draw_data = DrawData::new();

    crate::run::run_app::<T>(assets_path, map, sprite_flags, sprite_sheet, draw_data);

    Ok(())
}

pub struct Resources {
    pub assets_path: String,
    pub sprite_sheet: SpriteSheet,
    pub sprite_flags: Flags,
    pub map: Map,
}

/* UTILS */
pub(crate) fn write_and_log(file_name: &str, contents: &str) {
    print!("Writing {file_name}... ");
    std::fs::write(&file_name, contents).unwrap();
    println!("success.")
}

/*Pico8 math functions */
/// https://pico-8.fandom.com/wiki/Sin
pub fn sin(f: f32) -> f32 {
    (-f * 2.0 * PI).sin()
}

#[cfg(test)]
mod tests {
    use super::sin;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d && $y - $x < $d) {
                panic!();
            }
        };
    }

    #[allow(clippy::approx_constant)]
    #[test]
    fn sin_works() {
        assert_delta!(sin(0.0), 0.0, 0.00001);
        assert_delta!(sin(0.125), -0.70710677, 0.00001);
        assert_delta!(sin(0.25), -1.0, 0.00001);
        assert_delta!(sin(0.375), -0.70710677, 0.00001);
        assert_delta!(sin(0.5), 0.0, 0.00001);
        assert_delta!(sin(0.625), 0.70710677, 0.00001);
        assert_delta!(sin(0.75), 1.0, 0.00001);
        assert_delta!(sin(0.875), 0.70710677, 0.00001);
        assert_delta!(sin(1.0), 0.0, 0.00001);
    }
}
