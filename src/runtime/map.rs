use std::cell::Cell;

use super::sprite_sheet::Sprite;

type SpriteId = u8;
#[derive(Debug, Clone)]
pub struct Map {
    // Don't really want the size to change
    map: [Cell<SpriteId>; Self::MAP_SIZE],
}

impl Map {
    const SCREEN_SIZE_PIXELS: usize = 128;
    const SCREENS: usize = 4;
    const SPRITES_PER_SCREEN_ROW: usize = Self::SCREEN_SIZE_PIXELS / Sprite::WIDTH;
    pub const WIDTH_SPRITES: usize = Self::SCREENS * Self::SPRITES_PER_SCREEN_ROW;
    pub const HEIGHT_SPRITES: usize = Self::SCREENS * Self::SPRITES_PER_SCREEN_ROW;
    const MAP_SIZE: usize = Self::WIDTH_SPRITES * Self::HEIGHT_SPRITES;

    // TODO: Make pub(crate)
    pub fn new() -> Self {
        let mut map: [Cell<SpriteId>; Self::MAP_SIZE] =
            vec![Cell::new(0); Self::MAP_SIZE].try_into().unwrap();

        map[0] = Cell::new(1);
        map[1] = Cell::new(1);
        map[2] = Cell::new(1);

        Map { map }
    }

    pub(crate) fn mget(&self, cel_x: usize, cel_y: usize) -> u8 {
        let index = cel_x + cel_y * Map::WIDTH_SPRITES;
        // TODO: Handle like pico8
        assert!(index <= self.map.len());

        self.map[index].get()
    }

    pub(crate) fn mset(&self, cel_x: usize, cel_y: usize, sprite: u8) {
        let index = cel_x + cel_y * Map::WIDTH_SPRITES;
        // TODO: Handle like pico8
        assert!(index <= self.map.len());

        self.map[index].set(sprite);
    }

    pub fn iter(&self) -> impl Iterator<Item = SpriteId> + '_ {
        self.map.iter().map(Cell::get)
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}
