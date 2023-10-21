use itertools::Itertools;

use crate::serialize::Serialize;
use crate::{colors, Color};

#[derive(Debug, Clone, Copy)]
pub struct OwnedSprite {
    buffer: [Color; Sprite::WIDTH * Sprite::HEIGHT],
}

impl AsRef<Sprite> for OwnedSprite {
    fn as_ref(&self) -> &Sprite {
        Sprite::new(&self.buffer)
    }
}

impl AsMut<Sprite> for OwnedSprite {
    fn as_mut(&mut self) -> &mut Sprite {
        Sprite::new_mut(&mut self.buffer)
    }
}

impl OwnedSprite {
    fn black() -> Self {
        Self {
            buffer: [colors::BLACK; Sprite::WIDTH * Sprite::HEIGHT],
        }
    }

    fn set(&mut self, x: usize, y: usize, c: u8) {
        if x < Sprite::WIDTH && y < Sprite::HEIGHT {
            self.buffer[x + y * Sprite::WIDTH] = c;
        }
    }

    fn from_iterator(iter: impl Iterator<Item = Color>) -> Self {
        Self {
            buffer: iter.collect::<Vec<Color>>().try_into().unwrap(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Color> + '_ {
        self.buffer.iter().copied()
    }
}

/// A pico8 game's sprite sheet.
#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub(crate) sprite_sheet: Vec<OwnedSprite>,
}

impl SpriteSheet {
    pub fn file_name() -> String {
        "sprite_sheet.txt".to_owned()
    }
}

impl SpriteSheet {
    pub const SPRITES_PER_ROW: usize = 16;
    pub const ROWS_PER_PAGE: usize = 4;
    pub const PAGES: usize = 4;

    /// This is kind of a lie.
    /// The pico8 sprite sheet supports 128 "real" sprites
    /// The other 128 share memory with the map,
    /// and will override its data if used
    pub const SPRITE_COUNT: usize = Self::SPRITES_PER_ROW * Self::ROWS_PER_PAGE * Self::PAGES;

    pub fn new() -> Self {
        Self {
            sprite_sheet: vec![
                OwnedSprite::black();
                Self::SPRITE_COUNT * Sprite::WIDTH * Sprite::HEIGHT
            ],
        }
    }

    fn with_vec(sprite_sheet: Vec<OwnedSprite>) -> Result<Self, String> {
        const REQUIRED_SPRITES: usize = SpriteSheet::SPRITE_COUNT;

        if sprite_sheet.len() != REQUIRED_SPRITES {
            Err(format!(
                "[SpriteSheet] Needed {} sprites, got {}",
                REQUIRED_SPRITES,
                sprite_sheet.len()
            ))
        } else {
            Ok(Self { sprite_sheet })
        }
    }

    /// Sets the pixel at coordinate (x,y) in the spritesheet to a specified color
    pub fn set(&mut self, x: usize, y: usize, c: Color) {
        if c > 16 {
            panic!("Color too big {c}!");
        }

        let (sprite_x, sprite_y) = Self::sprite_index_from_pixel_index(x, y);
        let sprite_index = sprite_x + sprite_y * Self::SPRITES_PER_ROW;
        let sprite = &mut self.sprite_sheet[sprite_index];

        let (px, py) = (x - sprite_x * Sprite::WIDTH, y - sprite_y * Sprite::HEIGHT);
        sprite.set(px, py, c);
        // self.sprite_sheet[Self::to_linear_index(x, y)] = c;
    }

    fn sprite_index_from_pixel_index(px: usize, py: usize) -> (usize, usize) {
        let x = px / Sprite::WIDTH;
        let y = py / Sprite::HEIGHT;

        (x, y)
    }

    pub fn to_linear_index(x: usize, y: usize) -> usize {
        let x_part = 64 * (x / 8) + x % 8;
        let y_part = 16 * 64 * (y / 8) + 8 * (y % 8);

        y_part + x_part
    }

    pub fn get_sprite(&self, sprite: usize) -> &Sprite {
        self.sprite_sheet[sprite].as_ref()
    }

    pub fn get_sprite_mut(&mut self, sprite: usize) -> &mut Sprite {
        self.sprite_sheet[sprite].as_mut()
    }

    fn sprite_index(&self, sprite: usize) -> usize {
        // How many pixels we need to skip to get to the start of this sprite.
        sprite * Sprite::WIDTH * Sprite::HEIGHT
    }

    pub fn deserialize(str: &str) -> Result<Self, String> {
        let sprite_sheet = str
            .as_bytes()
            .iter()
            .copied()
            // TODO: Error out if any of these are errors
            .filter_map(|c| (c as char).to_digit(16))
            .map(|c| c as u8)
            .chunks(Sprite::SIZE)
            .into_iter()
            .map(|chunk| OwnedSprite::from_iterator(chunk))
            .collect();

        Self::with_vec(sprite_sheet)
    }
}

impl Default for SpriteSheet {
    fn default() -> Self {
        Self::new()
    }
}

impl Serialize for SpriteSheet {
    fn serialize(&self) -> String {
        // let lines = self.sprite_sheet.chunks(128).map(|chunk| {
        //     Itertools::intersperse(chunk.iter().map(|n| format!("{n:X}")), "".to_owned()).collect()
        // });

        // Itertools::intersperse(lines, "\n".to_owned()).collect::<String>()

        "TODO: Whoops, fix serialize".to_owned()
    }
}

#[repr(transparent)]
pub struct Sprite {
    pub sprite: [Color],
}

impl Sprite {
    pub const WIDTH: usize = 8;
    pub const HEIGHT: usize = 8;
    /// Total amount of pixels taken up by this sprite.
    /// Equals `WIDTH * HEIGHT`.
    pub const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    pub fn new(sprite: &[u8]) -> &Self {
        unsafe { &*(sprite as *const [u8] as *const Self) }
    }

    pub fn to_owned(&self) -> Vec<Color> {
        let sprite = &self.sprite;

        sprite.to_vec()
    }

    pub fn new_mut(sprite: &mut [u8]) -> &mut Self {
        unsafe { &mut *(sprite as *mut [u8] as *mut Self) }
    }

    pub(crate) fn set(&mut self, index: usize, color: Color) {
        self.sprite[index] = color;
    }

    pub fn pset(&mut self, x: isize, y: isize, color: Color) {
        // TODO: is unwrapping here ok? Why?
        if let Some(index) = Self::index(x, y) {
            self.set(index, color);
        }
    }

    pub fn pget(&self, x: isize, y: isize) -> Color {
        self.sprite[Self::index(x, y).unwrap()]
    }

    fn index(x: isize, y: isize) -> Option<usize> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        if x < Sprite::WIDTH && y < Sprite::HEIGHT {
            Some(x + y * Sprite::WIDTH)
        } else {
            None
        }
    }

    pub fn shift_up(&mut self) {
        self.sprite.rotate_left(8);
    }

    pub fn shift_down(&mut self) {
        self.sprite.rotate_right(8);
    }

    pub fn shift_left(&mut self) {
        self.sprite
            .chunks_mut(Sprite::WIDTH)
            .for_each(|row| row.rotate_left(1));
    }

    pub fn shift_right(&mut self) {
        self.sprite
            .chunks_mut(Sprite::WIDTH)
            .for_each(|row| row.rotate_right(1));
    }

    pub fn iter(&self) -> impl Iterator<Item = Color> + '_ {
        self.sprite.iter().copied()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Color> + '_ {
        self.sprite.iter_mut()
    }

    pub fn flip_horizontally(&mut self) {
        for row in self.sprite.chunks_mut(Self::WIDTH) {
            row.reverse()
        }
    }

    pub fn flip_vertically(&mut self) {
        for x in 0..(Self::WIDTH as isize) {
            for y in 0..((Self::HEIGHT / 2) as isize) {
                self.sprite.swap(
                    Self::index(x, y).unwrap(),
                    Self::index(x, Self::HEIGHT as isize - 1 - y).unwrap(),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexing_works() {
        assert_eq!(SpriteSheet::to_linear_index(7, 0), 7);
        assert_eq!(SpriteSheet::to_linear_index(0, 1), 8);
        assert_eq!(SpriteSheet::to_linear_index(8, 0), 64);
        assert_eq!(SpriteSheet::to_linear_index(8, 1), 64 + 8);
        assert_eq!(SpriteSheet::to_linear_index(1, 9), 1033);
    }
}
