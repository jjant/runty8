pub type Color = u8; // Actually a u4

use itertools::Itertools;
use std::{fs::File, io::Read};

#[derive(Debug)]
pub(crate) struct SpriteSheet {
    pub(crate) sprite_sheet: Vec<Color>,
}

impl SpriteSheet {
    pub const SPRITE_COUNT: usize = 256;

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            sprite_sheet: vec![0; Self::SPRITE_COUNT * Sprite::WIDTH * Sprite::HEIGHT],
        }
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

    pub fn serialize(&self) -> String {
        let lines = self.sprite_sheet.chunks(128).map(|chunk| {
            Itertools::intersperse(chunk.iter().map(|n| format!("{:X}", n)), "".to_owned())
                .collect()
        });

        Itertools::intersperse(lines, "\n".to_owned()).collect::<String>()
    }

    pub fn deserialize(str: &str) -> Self {
        let sprite_sheet = str
            .as_bytes()
            .iter()
            .copied()
            .filter_map(|c| (c as char).to_digit(16))
            .map(|c| c as u8)
            .collect();

        Self { sprite_sheet }
    }
}

// TODO: Make a more reliable version of this.
// TODO: Improve capacity calculation? It's kinda flakey
pub(crate) fn deserialize() -> SpriteSheet {
    let capacity = 128 * 128 + 128;
    let mut file = String::with_capacity(capacity);
    File::open("sprite_sheet.txt")
        .expect("Couldn't OPEN file")
        .read_to_string(&mut file)
        .expect("Couldn't READ file");

    SpriteSheet::deserialize(&file)
}

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

    pub fn pset(&mut self, x: isize, y: isize, color: Color) {
        self.sprite[Self::index(x, y).unwrap()] = color;
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
}
