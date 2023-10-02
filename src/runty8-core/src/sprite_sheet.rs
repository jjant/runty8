use itertools::Itertools;

use crate::serialize::Serialize;
use crate::Color;

/// A pico8 game's sprite sheet.
#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub(crate) sprite_sheet: Vec<Color>,
}

impl SpriteSheet {
    pub fn file_name() -> String {
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

    pub fn to_linear_index(x: usize, y: usize) -> usize {
        let x_part = 64 * (x / 8) + x % 8;
        let y_part = 16 * 64 * (y / 8) + 8 * (y % 8);

        y_part + x_part
    }

    pub fn get_sprite(&self, sprite: usize) -> &Sprite {
        let index = self.sprite_index(sprite);

        Sprite::new(&self.sprite_sheet[index..(index + Sprite::WIDTH * Sprite::HEIGHT)])
    }

    pub fn get_sprite_mut(&mut self, sprite: usize) -> &mut Sprite {
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

impl Default for SpriteSheet {
    fn default() -> Self {
        Self::new()
    }
}

impl Serialize for SpriteSheet {
    fn serialize(&self) -> String {
        let lines = self.sprite_sheet.chunks(128).map(|chunk| {
            Itertools::intersperse(chunk.iter().map(|n| format!("{n:X}")), "".to_owned()).collect()
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

    pub fn to_clipboard_string(&self) -> String {
        let width = Self::WIDTH;
        let height = Self::HEIGHT;
        let pixel_data = self.to_owned();

        let mut str = String::new();
        for pixel in pixel_data {
            str.push(char::from_digit(pixel as u32, 16).unwrap());
        }

        // TODO: Google how to pad width/height to 2 characters with zeros.
        format!("[gfx]0{width:x}0{height:x}{str}[/gfx]")
    }

    pub fn from_clipboard(str: &str) -> Result<Vec<Color>, String> {
        let str = chomp(str, "[gfx]")?;
        let (width, str) = chomp_int(str, 2)?;
        let (height, str) = chomp_int(str, 2)?;
        let mut data = Vec::with_capacity(width as usize * height as usize);

        let data_size = {
            let expected_data_size = (width * height) as usize;
            let actual_data_size = str.len() as i64 - "[/gfx]".len() as i64;

            if expected_data_size as i64 == actual_data_size {
                Ok(actual_data_size as usize)
            } else {
                let actual_len = str.len() - "[/gfx]".len();

                Err(format!(
                    "Expected data section to be {expected_data_size} long, but it is {actual_len}",
                ))
            }
        }?;

        let (data_str, str) = str.split_at(data_size);

        for char in data_str.chars() {
            let digit = char
                .to_digit(16)
                .ok_or_else(|| format!("{char} is not a hex digit"))? as u8;
            data.push(digit);
        }
        let _ = chomp(str, "[/gfx]")?;

        Ok(data)
    }
}

fn chomp<'a>(source: &'a str, chomped: &str) -> Result<&'a str, String> {
    if source.starts_with(chomped) {
        Ok(&source[chomped.len()..])
    } else {
        Err(format!("\"{source}\" doesn't start with \"{chomped}\""))
    }
}

fn chomp_int(source: &str, len: usize) -> Result<(i32, &str), String> {
    let (int_str, next) = source.split_at(len);

    if let Ok(int) = int_str.parse() {
        Ok((int, next))
    } else {
        Err(format!("Couldn't parse int: {int_str}"))
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

    #[test]
    fn clipboard_works() {
        let str_sprite =
            "[gfx]0808fff00000fff00000fff000000000000000000000000000000000000000000000[/gfx]";

        let mut arr = [0; Sprite::WIDTH * Sprite::HEIGHT];
        let sprite = { Sprite::new_mut(&mut arr) };

        for (x, y) in (0..3).cartesian_product(0..3) {
            sprite.set(x + y * Sprite::WIDTH, 15);
        }

        assert_eq!(&sprite.to_clipboard_string(), str_sprite);
        assert_eq!(
            Sprite::from_clipboard(str_sprite).unwrap(),
            sprite.to_owned()
        );
    }

    #[test]
    fn from_clipboard_with_wrong_size() {
        let str_sprite = "[gfx]0808ff00[/gfx]";

        let actual = Sprite::from_clipboard(str_sprite);
        assert!(actual.is_err());
    }
}
