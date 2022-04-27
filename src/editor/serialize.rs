use crate::SpriteSheet;
use crate::COLORS;
use std::io::Write;
use std::{fs::File, path::Path};

pub(crate) struct Ppm {
    height: u32,
    width: u32,
    data: Vec<u8>,
}

impl Ppm {
    pub fn from_sprite_sheet(sprite_sheet: &SpriteSheet) -> Self {
        Self {
            width: 16 * 8,
            height: 4 * 4 * 8,
            data: sprite_sheet
                .sprite_sheet
                .iter()
                .flat_map(|c| {
                    let c = COLORS[*c as usize];
                    let r = ((c >> 16) & 0x0000FF) as u8;
                    let g = ((c >> 8) & 0x0000FF) as u8;
                    let b = (c & 0x0000FF) as u8;

                    [r, g, b]
                })
                .collect(),
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
