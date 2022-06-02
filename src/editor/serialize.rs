use itertools::Itertools;

use crate::runtime::draw_context::COLORS;
use crate::runtime::map::Map;
use crate::runtime::sprite_sheet::SpriteSheet;
use std::fmt::Display;
use std::io::Write;
use std::{fs::File, path::Path};

pub fn serialize_named<T: Serialize>(assets_path: &str, file_name: &str, serializable: &T) {
    let file_path = format!("{assets_path}/{}", file_name);
    crate::write_and_log(&file_path, &serializable.serialize());
}

pub fn serialize<T: Serialize>(assets_path: &str, serializable: &T) {
    serialize_named(assets_path, &T::file_name(), serializable);
}

pub trait Serialize {
    fn file_name() -> String;

    fn serialize(&self) -> String;
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn from_pico8(color_index: u8) -> Self {
        let c = COLORS[color_index as usize];
        let r = ((c >> 16) & 0x0000FF) as u8;
        let g = ((c >> 8) & 0x0000FF) as u8;
        let b = (c & 0x0000FF) as u8;

        Self { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.r, self.g, self.b))
    }
}
pub struct Ppm {
    height: usize,
    width: usize,
    data: Vec<Color>,
}

const SPRITE_PAGES: usize = 4;
const ROWS_PER_PAGE: usize = 4;

impl Ppm {
    pub fn from_map(map: &Map, sprite_sheet: &SpriteSheet) -> Self {
        let width = 1024;
        let height = 4 * 16 * 8;
        let mut data = vec![Color { r: 0, g: 0, b: 0 }; width * height];

        for (y, row) in map.map.iter().chunks(128).into_iter().enumerate() {
            for (x, sprite_id) in row.into_iter().copied().enumerate() {
                let real_x = x * 8;
                let real_y = y * 8;

                let sprite = sprite_sheet.get_sprite(sprite_id as usize);

                for (pixel_index, pixel) in sprite.iter().enumerate() {
                    let offset_x = pixel_index % 8;
                    let offset_y = pixel_index / 8;

                    let color = Color::from_pico8(pixel);

                    data[(real_x + offset_x) + (real_y + offset_y) * width] = color;
                }
            }
        }

        Self {
            width,
            height,
            data,
        }
    }

    pub fn from_sprite_sheet(sprite_sheet: &SpriteSheet) -> Self {
        let sprite_sheet = &sprite_sheet.sprite_sheet;
        let width = SpriteSheet::SPRITES_PER_ROW as usize * SPRITE_WIDTH;
        let height = SPRITE_PAGES * ROWS_PER_PAGE * SPRITE_WIDTH;

        let mut data = vec![Color { r: 0, g: 0, b: 0 }; sprite_sheet.len()];

        const SPRITE_WIDTH: usize = 8;
        const SPRITE_SIZE: usize = SPRITE_WIDTH.pow(2);

        #[allow(clippy::never_loop)]
        for (sprite_index, sprite) in sprite_sheet
            .iter()
            .copied()
            .chunks(SPRITE_SIZE)
            .into_iter()
            .enumerate()
        {
            let base_x = SPRITE_WIDTH * (sprite_index % SpriteSheet::SPRITES_PER_ROW as usize);
            let base_y = SPRITE_WIDTH * (sprite_index / SpriteSheet::SPRITES_PER_ROW as usize);

            for (pixel_index, c) in sprite.enumerate() {
                let x = base_x + pixel_index % SPRITE_WIDTH;
                let y = base_y + pixel_index / SPRITE_WIDTH;

                let color = Color::from_pico8(c);
                data[(x + y * 128)] = color;
            }
        }

        Self {
            width,
            height,
            data,
        }
    }

    // Raw PPM format (P6)
    pub fn write_file(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write_all(header.as_bytes())?;

        let len = std::mem::size_of_val(&self.data as &[Color]);
        let slice = unsafe { std::slice::from_raw_parts(self.data.as_ptr().cast(), len) };
        // file.write_all(&self.data)?;
        file.write_all(slice)?;
        Ok(())
    }
}

impl Serialize for Ppm {
    fn file_name() -> String {
        "sprite_sheet.ppm".to_owned()
    }

    // Plain PPM format (P3)
    fn serialize(&self) -> String {
        let header = format!("P3\n{} {}\n255", self.width, self.height);
        let body = self
            .data
            .iter()
            .copied()
            .map(|component| format!(" {} ", component))
            .join("");
        // let body = self
        //     .data
        //     .iter()
        //     .copied()
        //     .chunks(self.width)
        //     .into_iter()
        //     // .map(|component| component.to_string())
        //     .map(|chunk| chunk.map(|c| c.to_string()).join(" "))
        //     .join("\n");

        format!("{header}\n{body}")
    }
}
