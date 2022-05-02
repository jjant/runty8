pub mod app;
mod draw;
mod editor;
mod font;
pub mod graphics;
pub mod runtime;
mod screen;
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

#[repr(transparent)]
#[derive(Debug)]
pub(crate) struct Map {
    // Don't really want the size to change
    map: Box<[u8]>,
}

impl Map {
    const SCREEN_SIZE_PIXELS: usize = 128;
    const SCREENS: usize = 4;
    const SPRITES_PER_SCREEN_ROW: usize = Self::SCREEN_SIZE_PIXELS / Sprite::WIDTH;
    pub const WIDTH_SPRITES: usize = Self::SCREENS * Self::SPRITES_PER_SCREEN_ROW;
    pub const HEIGHT_SPRITES: usize = Self::SCREENS * Self::SPRITES_PER_SCREEN_ROW;
    const MAP_SIZE: usize = Self::WIDTH_SPRITES * Self::HEIGHT_SPRITES;

    fn new() -> Self {
        let mut map = vec![0; Self::MAP_SIZE].into_boxed_slice();

        map[0] = 1;
        map[1] = 1;
        map[2] = 1;

        Map { map }
    }

    fn mget(&self, cel_x: usize, cel_y: usize) -> u8 {
        let index = cel_x + cel_y * Map::WIDTH_SPRITES;
        // TODO: Handle like pico8
        assert!(index <= self.map.len());

        self.map[index]
    }

    fn mset(&mut self, cel_x: usize, cel_y: usize, sprite: u8) {
        let index = cel_x + cel_y * Map::WIDTH_SPRITES;
        // TODO: Handle like pico8
        assert!(index <= self.map.len());

        self.map[index] = sprite;
    }
}
