pub type Color = u8; // Actually a u4

use itertools::Itertools;

use crate::editor::serialize::Serialize;

#[derive(Debug)]
pub struct SpriteSheet {
    pub(crate) sprite_sheet: Vec<Color>,
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

    pub fn set(&mut self, x: usize, y: usize, c: Color) {
        self.sprite_sheet[Self::to_linear_index(x, y)] = c;
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
    fn file_name() -> String {
        "sprite_sheet.txt".to_owned()
    }

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

    fn new_mut(sprite: &mut [u8]) -> &mut Self {
        unsafe { &mut *(sprite as *mut [u8] as *mut Self) }
    }

    pub(crate) fn set(&mut self, index: usize, color: Color) {
        self.sprite[index] = color;
    }

    pub fn pset(&mut self, x: isize, y: isize, color: Color) {
        // TODO: is unwrapping here ok? Why?
        let index = Self::index(x, y).unwrap();

        self.set(index, color);
    }

    pub fn pget(&self, x: isize, y: isize) -> Color {
        self.sprite[Self::index(x, y).unwrap()]
    }

    fn index(x: isize, y: isize) -> Option<usize> {
        (x + y * (Sprite::WIDTH as isize)).try_into().ok()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexing_works() {
        assert_eq!(SpriteSheet::to_linear_index(8, 0), 64);
        assert_eq!(SpriteSheet::to_linear_index(8, 1), 64 + 8);
        assert_eq!(SpriteSheet::to_linear_index(1, 9), 1033);
    }
}
