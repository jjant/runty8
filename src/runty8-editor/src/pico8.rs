//! Extension trait to render editor sprites.

use once_cell::sync::Lazy;
use runty8_core::{Pico8, SpriteSheet};

/// Extension trait to render editor sprites.
pub(crate) trait Pico8EditorExt {
    fn editor_spr(&mut self, spr: usize, x: i32, y: i32);
}

impl Pico8EditorExt for Pico8 {
    fn editor_spr(&mut self, spr: usize, x: i32, y: i32) {
        let sprite = editor_sprite_sheet().get_sprite(spr);

        self.raw_spr(sprite, x, y, 1.0, 1.0, false, false);
    }
}

/// Lazily parse sprite sheet.
fn editor_sprite_sheet() -> &'static SpriteSheet {
    static SPRITE_SHEET: Lazy<SpriteSheet> = Lazy::new(|| {
        SpriteSheet::deserialize(include_str!("./editor_assets/sprite_sheet.txt")).unwrap()
    });

    &SPRITE_SHEET
}
