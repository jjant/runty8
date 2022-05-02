pub mod app;
mod draw;
mod editor;
mod font;
pub mod graphics;
pub mod runtime;
pub mod screen;
pub mod ui;
pub use app::App;
use runtime::{
    sprite_sheet::{Color, Sprite},
    state::State,
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
