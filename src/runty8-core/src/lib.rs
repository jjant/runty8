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
pub use input::Keys;
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
    /// Rigth arrow.
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
    Down(MouseButton),
    /// Mouse button released.
    Up(MouseButton),
}

/// Keyboard events (key up, key down).
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
    Tick { delta_millis: f64 },
}
