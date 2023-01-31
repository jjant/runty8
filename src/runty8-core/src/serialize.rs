use itertools::Itertools;

use crate::draw_data::COLORS;
use crate::map::Map;
use crate::sprite_sheet::SpriteSheet;
use std::fmt::Display;

fn write_and_log(file_name: &str, contents: &str) {
    log::info!("Writing {file_name}... ");
    write(file_name, contents).unwrap();
    log::info!("success.");
}

#[cfg(not(target_arch = "wasm32"))]
type WriteError = std::io::Error;
#[cfg(target_arch = "wasm32")]
type WriteError = wasm::Error;

/// Stores this file:
///  - Native: Uses regular `std::fs::write`
///  - Web: Uses `localStorage.setItem`
fn write(file_name: &str, contents: &str) -> Result<(), WriteError> {
    #[cfg(not(target_arch = "wasm32"))]
    return std::fs::write(file_name, contents);
    #[cfg(target_arch = "wasm32")]
    return wasm::write(file_name, contents);
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    pub(super) type Error = wasm_bindgen::JsValue;

    pub(super) fn write(key: &str, contents: &str) -> Result<(), Error> {
        let storage = web_sys::window()
            .expect("Couldn't access window object")
            .local_storage()
            .expect("Couldn't access local storage object")
            .expect("Couldn't access local storage object");

        storage.set_item(key, contents)
    }
}

pub fn serialize(assets_path: &str, file_name: &str, serializable: &impl Serialize) {
    let file_path = format!("{assets_path}/{file_name}");

    write_and_log(&file_path, &serializable.serialize());
}

pub trait Serialize {
    fn serialize(&self) -> String;
}

impl<T: Serialize + ?Sized> Serialize for &T {
    fn serialize(&self) -> String {
        T::serialize(self)
    }
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

const SPRITE_PAGES: usize = 4;
const ROWS_PER_PAGE: usize = 4;

/// Utility to create PPM images.
/// Useful for debugging our data structures (sprite sheet, map)
/// in regular image viewers.
pub struct Ppm {
    height: usize,
    width: usize,
    data: Vec<Color>,
}

impl Ppm {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn from_sprite_sheet(sprite_sheet: &SpriteSheet) -> Self {
        let sprite_sheet = &sprite_sheet.sprite_sheet;
        let width = SpriteSheet::SPRITES_PER_ROW * SPRITE_WIDTH;
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
            let base_x = SPRITE_WIDTH * (sprite_index % SpriteSheet::SPRITES_PER_ROW);
            let base_y = SPRITE_WIDTH * (sprite_index / SpriteSheet::SPRITES_PER_ROW);

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
}

impl Serialize for Ppm {
    /// Plain PPM format (P3).
    fn serialize(&self) -> String {
        let header = format!("P3\n{} {}\n255", self.width, self.height);
        let body = self
            .data
            .iter()
            .copied()
            .map(|component| format!(" {component} "))
            .join("");

        format!("{header}\n{body}")
    }
}

/// Represents a serialized asset ready to be stored to a file.
pub struct Serialized {
    pub file_name: String,
    pub serialized: String,
}
