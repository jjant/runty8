use itertools::Itertools;

use crate::editor::SPRITES_PER_ROW;
use crate::runtime::draw_context::COLORS;
use crate::runtime::sprite_sheet::SpriteSheet;
use std::io::Write;
use std::{fs::File, path::Path};

pub(crate) struct Ppm {
    height: usize,
    width: usize,
    data: Vec<u8>,
}

const SPRITE_PAGES: usize = 4;
const ROWS_PER_PAGE: usize = 4;

impl Ppm {
    pub fn from_sprite_sheet(sprite_sheet: &SpriteSheet) -> Self {
        let sprite_sheet = &sprite_sheet.sprite_sheet;
        let width = SPRITES_PER_ROW as usize * SPRITE_WIDTH;
        let height = SPRITE_PAGES * ROWS_PER_PAGE * SPRITE_WIDTH;

        let mut data = vec![0; sprite_sheet.len() * 3];

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
            let base_x = SPRITE_WIDTH * (sprite_index % SPRITES_PER_ROW as usize);
            let base_y = SPRITE_WIDTH * (sprite_index / SPRITES_PER_ROW as usize);

            for (pixel_index, c) in sprite.enumerate() {
                let c = COLORS[c as usize];
                let r = ((c >> 16) & 0x0000FF) as u8;
                let g = ((c >> 8) & 0x0000FF) as u8;
                let b = (c & 0x0000FF) as u8;

                let x = base_x + pixel_index % SPRITE_WIDTH;
                let y = base_y + pixel_index / SPRITE_WIDTH;

                data[3 * (x + y * 128)] = r;
                data[3 * (x + y * 128) + 1] = g;
                data[3 * (x + y * 128) + 2] = b;
            }
        }

        Self {
            width,
            height,
            data,
        }
    }
    pub fn write_file(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write_all(header.as_bytes())?;
        file.write_all(&self.data)?;
        Ok(())
    }
}
