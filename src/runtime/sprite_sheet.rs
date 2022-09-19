pub type Color = u8; // Actually a u4

use itertools::Itertools;

use crate::editor::serialize::Serialize;

#[derive(Debug)]
pub(crate) struct SpriteSheet {
    pub(crate) sprite_sheet: Vec<Color>,
}

impl SpriteSheet {
    pub(crate) fn file_name() -> String {
        "sprite_sheet.txt".to_owned()
    }
}

impl SpriteSheet {
    pub const SPRITES_PER_ROW: usize = 16;

    /// This is kind of a lie.
    /// The pico8 sprite sheet supports 128 "real" sprites
    /// The other 128 share memory with the map,
    /// and will override its data if used
    pub const SPRITE_COUNT: usize = 256;

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            sprite_sheet: vec![0; Self::SPRITE_COUNT * Sprite::WIDTH * Sprite::HEIGHT],
        }
    }

    fn with_vec(sprite_sheet: Vec<Color>) -> Result<Self, String> {
        const REQUIRED_BYTES: usize = SpriteSheet::SPRITE_COUNT * Sprite::WIDTH * Sprite::HEIGHT;

        if sprite_sheet.len() != REQUIRED_BYTES {
            Err(format!(
                "[SpriteSheet] Needed {} bytes, got {}",
                REQUIRED_BYTES,
                sprite_sheet.len()
            ))
        } else {
            Ok(Self { sprite_sheet })
        }
    }

    /// Sets the pixel at coordinate (x,y) in the spritesheet to a specified color
    pub fn set(&mut self, x: usize, y: usize, c: Color) {
        self.sprite_sheet[Self::to_linear_index(x, y)] = c;
    }

    /// Converts (x, y) sprite coordinates into the sprite index if within bounds.
    /// The sprite index is what's needed for functions like `Pico8::spr`.
    pub(crate) fn coords_from_sprite_index(index: usize) -> (usize, usize) {
        (index % Self::SPRITES_PER_ROW, index / Self::SPRITES_PER_ROW)
    }

    /// Converts (x, y) sprite coordinates into the sprite index if within bounds.
    /// The sprite index is what's needed for functions like `Pico8::spr`.
    pub(crate) fn sprite_index_from_coords(x: usize, y: usize) -> Option<usize> {
        if x >= 16 || y >= 16 {
            None
        } else {
            Some(x + y * 16)
        }
    }

    pub fn to_linear_index(x: usize, y: usize) -> usize {
        let x_part = 64 * (x / 8) + x % 8;
        let y_part = 16 * 64 * (y / 8) + 8 * (y % 8);

        y_part + x_part
    }

    pub fn get_sprite(&self, sprite: usize) -> &Sprite {
        let index = self.sprite_index(sprite);

        Sprite::new(&self.sprite_sheet[index..(index + Sprite::WIDTH * Sprite::HEIGHT)])
    }

    pub(crate) fn get_sprite_mut(&mut self, sprite: usize) -> &mut Sprite {
        let index = self.sprite_index(sprite);

        Sprite::new_mut(&mut self.sprite_sheet[index..(index + Sprite::WIDTH * Sprite::HEIGHT)])
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
            .filter_map(|c| (c as char).to_digit(16))
            .map(|c| c as u8)
            .collect();

        Self::with_vec(sprite_sheet)
    }
}

impl Serialize for SpriteSheet {
    fn serialize(&self) -> String {
        let lines = self.sprite_sheet.chunks(128).map(|chunk| {
            Itertools::intersperse(chunk.iter().map(|n| format!("{:X}", n)), "".to_owned())
                .collect()
        });

        Itertools::intersperse(lines, "\n".to_owned()).collect::<String>()
    }
}

#[repr(transparent)]
pub struct Sprite {
    pub sprite: [Color],
}

impl Sprite {
    pub const WIDTH: usize = 8;
    pub const HEIGHT: usize = 8;

    pub(crate) fn new(sprite: &[u8]) -> &Self {
        unsafe { &*(sprite as *const [u8] as *const Self) }
    }

    pub fn to_owned(&self) -> Vec<Color> {
        let sprite = &self.sprite;

        sprite.to_vec()
    }

    fn new_mut(sprite: &mut [u8]) -> &mut Self {
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

    pub(crate) fn shift_up(&mut self) {
        self.sprite.rotate_left(8);
    }

    pub(crate) fn shift_down(&mut self) {
        self.sprite.rotate_right(8);
    }

    pub(crate) fn shift_left(&mut self) {
        self.sprite
            .chunks_mut(Sprite::WIDTH)
            .for_each(|row| row.rotate_left(1));
    }

    pub(crate) fn shift_right(&mut self) {
        self.sprite
            .chunks_mut(Sprite::WIDTH)
            .for_each(|row| row.rotate_right(1));
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = Color> + '_ {
        self.sprite.iter().copied()
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut Color> + '_ {
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
    fn sprite_index_from_coords_works() {
        assert_eq!(SpriteSheet::sprite_index_from_coords(0, 0), Some(0));
        assert_eq!(SpriteSheet::sprite_index_from_coords(0, 1), Some(16));
        assert_eq!(SpriteSheet::sprite_index_from_coords(8, 0), Some(8));
        assert_eq!(SpriteSheet::sprite_index_from_coords(8, 1), Some(24));
        assert_eq!(SpriteSheet::sprite_index_from_coords(8, 7), Some(120));
        assert_eq!(SpriteSheet::sprite_index_from_coords(16, 9), None);
        assert_eq!(SpriteSheet::sprite_index_from_coords(1, 16), None);
    }

    #[test]
    fn coords_from_sprite_index_works() {
        assert_eq!(SpriteSheet::coords_from_sprite_index(0), (0, 0));
        assert_eq!(SpriteSheet::coords_from_sprite_index(1), (1, 0));
        assert_eq!(SpriteSheet::coords_from_sprite_index(15), (15, 0));
        assert_eq!(SpriteSheet::coords_from_sprite_index(16), (0, 1));
        assert_eq!(SpriteSheet::coords_from_sprite_index(17), (1, 1));
        assert_eq!(SpriteSheet::coords_from_sprite_index(109), (13, 6));
        assert_eq!(SpriteSheet::coords_from_sprite_index(117), (5, 7));
        assert_eq!(SpriteSheet::coords_from_sprite_index(255), (15, 15));
    }

    #[test]
    fn sprite_index_coords_roundtrip() {
        for spr in 0..=255 {
            let (x, y) = SpriteSheet::coords_from_sprite_index(spr);
            let roundtrip_spr = SpriteSheet::sprite_index_from_coords(x, y);

            assert_eq!(Some(spr), roundtrip_spr);
        }
    }

    #[test]
    fn indexing_works() {
        assert_eq!(SpriteSheet::to_linear_index(7, 0), 7);
        assert_eq!(SpriteSheet::to_linear_index(0, 1), 8);
        assert_eq!(SpriteSheet::to_linear_index(8, 0), 64);
        assert_eq!(SpriteSheet::to_linear_index(8, 1), 64 + 8);
        assert_eq!(SpriteSheet::to_linear_index(1, 9), 1033);
    }
}
