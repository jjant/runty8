use crate::serialize::Serialize;

use super::sprite_sheet::Sprite;
use itertools::Itertools;

type SpriteId = u8;

/// A pico8 game's sprite map.
#[derive(Debug, Clone)]
pub struct Map {
    // Don't really want the size to change
    pub(crate) map: [SpriteId; Self::MAP_SIZE],
}

impl Map {
    pub fn file_name() -> String {
        "map.txt".to_owned()
    }
}

impl Map {
    const SCREEN_SIZE_PIXELS: usize = 128;
    const SCREENS_WIDTH: usize = 8; // map is 8 screens wide
    const SCREENS_HEIGHT: usize = 4; // map is 4 screens tall (actually 2, bottom 2 are shared with spritesheet)

    const SPRITES_PER_SCREEN_ROW: usize = Self::SCREEN_SIZE_PIXELS / Sprite::WIDTH;
    pub const WIDTH_SPRITES: usize = Self::SCREENS_WIDTH * Self::SPRITES_PER_SCREEN_ROW;
    pub const HEIGHT_SPRITES: usize = Self::SCREENS_HEIGHT * Self::SPRITES_PER_SCREEN_ROW;
    const MAP_SIZE: usize = Self::WIDTH_SPRITES * Self::HEIGHT_SPRITES;

    pub const WIDTH_PIXELS: usize = Self::WIDTH_SPRITES * Sprite::WIDTH;
    pub const HEIGHT_PIXELS: usize = Self::HEIGHT_SPRITES * Sprite::WIDTH;

    // TODO: Make pub(crate)
    pub fn new() -> Self {
        let mut map = [0; Self::MAP_SIZE];

        map[0] = 1;
        map[1] = 1;
        map[2] = 1;

        Map { map }
    }

    pub fn mget(&self, cel_x: i32, cel_y: i32) -> u8 {
        let index = Self::index(cel_x, cel_y);

        // TODO: Handle like pico8
        // TODO2: I think it returns 0 if outside bounds?
        index.map(|index| self.map[index]).unwrap_or(0)
    }

    pub fn mset(&mut self, cel_x: usize, cel_y: usize, sprite: u8) {
        let index = cel_x + cel_y * Map::WIDTH_SPRITES;
        // TODO: Handle like pico8
        assert!(index <= self.map.len());

        self.map[index] = sprite;
    }

    fn index(x: i32, y: i32) -> Option<usize> {
        if x >= 0
            && (x as usize) < Map::WIDTH_SPRITES
            && y >= 0
            && (y as usize) < Map::HEIGHT_SPRITES
        {
            Some(x as usize + y as usize * Map::WIDTH_SPRITES)
        } else {
            None
        }
    }
}

impl Map {
    // TODO: Make sure this works
    pub fn deserialize(str: &str) -> Result<Self, String> {
        let map: [SpriteId; Self::MAP_SIZE] = str
            .split_ascii_whitespace()
            .map(|num| u8::from_str_radix(num, 16).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|error: Vec<u8>| format!("Error deserializing map {}", error.len()))?;

        Ok(Self { map })
    }
}

impl Serialize for Map {
    // TODO: Make sure this works
    fn serialize(&self) -> String {
        self.map
            .iter()
            .chunks(Map::WIDTH_SPRITES)
            .into_iter()
            .map(|chunk| chunk.map(|n| format!("{n:0>2X}")).join(" "))
            .join("\n")
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}
