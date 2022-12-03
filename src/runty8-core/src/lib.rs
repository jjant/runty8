//! Types and functions required to run a Runty8 game.
mod draw_data;
mod flags;
mod input;
mod map;
mod pico8;
pub mod serialize;
mod sprite_sheet;
mod state;
pub use draw_data::colors;

pub mod draw;
pub mod font;

pub use flags::Flags;
pub use input::Input;
pub use map::Map;
pub use pico8::*;
pub use sprite_sheet::{Sprite, SpriteSheet};

/// A regular pico8 app.
pub trait App {
    fn init(pico8: &mut Pico8) -> Self;
    fn update(&mut self, pico8: &mut Pico8);
    fn draw(&mut self, pico8: &mut Pico8);
}

/// A pico8 color.
///
/// Valid colors are in the range `0..=15`.
pub type Color = u8; // Actually a u4

/// Pico8's supported input buttons.
#[derive(Debug)]
pub enum Button {
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// X key.
    X,
    /// C key.
    C,
    /// Left mouse button.
    Mouse,
}

/// Game assets: sprite sheet, map, flags.
#[derive(Debug)]
pub struct Resources {
    pub assets_path: String,
    pub sprite_sheet: SpriteSheet,
    pub sprite_flags: Flags,
    pub map: Map,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Key state: up or down.
pub enum KeyState {
    Up,
    Down,
}
/// Keyboard keys.
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
    Alt,
    Space,
}

/// Keyboard event (key up/down).
#[derive(Clone, Copy, Debug)]
pub struct KeyboardEvent {
    pub key: Key,
    pub state: KeyState,
}

#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
}

/// Mouse buttons.
#[derive(Clone, Copy, Debug)]
pub enum MouseButton {
    // TODO: Handle other mouse buttons?
    Left,
    Right,
    Middle,
}

/// Mouse events (mouse move, button presses).
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
    /// Mouse button pressed.
    Button {
        button: MouseButton,
        state: KeyState,
    },
}

/// Runty8 events (input, tick, etc).
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Input(InputEvent),
    Tick { delta_millis: f64 },
    WindowClosed,
}

/// Embed game assets in your binary (that is, loading them at compile time).
#[macro_export]
macro_rules! load_assets {
    ($path:tt) => {{
        static MAP_BYTES: &str = include_str!(concat!($path, "/map.txt"));
        static FLAGS_BYTES: &str = include_str!(concat!($path, "/sprite_flags.txt"));
        static SPRITE_SHEET_BYTES: &str = include_str!(concat!($path, "/sprite_sheet.txt"));

        let map = $crate::Map::deserialize(MAP_BYTES).unwrap();
        let sprite_flags = $crate::Flags::deserialize(FLAGS_BYTES).unwrap();
        let sprite_sheet = $crate::SpriteSheet::deserialize(SPRITE_SHEET_BYTES).unwrap();

        $crate::Resources {
            map,
            sprite_flags,
            sprite_sheet,
            assets_path: $path.to_owned(),
        }
    }};
}
