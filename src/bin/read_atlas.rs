use std::{collections::HashMap, fs::File};

use runty8::editor::serialize::{serialize, Ppm};
use runty8::runtime::draw_context::COLORS;
use runty8::runtime::sprite_sheet::{Color, SpriteSheet};
use runty8::runtime::state::Flags;

const DIR_NAME: &str = "assets";

fn main() {
    let sprite_sheet = build_sprite_sheet();
    serialize(DIR_NAME, &sprite_sheet);

    let sprite_sheet_ppm = Ppm::from_sprite_sheet(&sprite_sheet);
    serialize(DIR_NAME, &sprite_sheet_ppm);

    let flags = build_flags();
    serialize(DIR_NAME, &flags);
}

/* FLAGS */
const FLAGS: [u8; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
    4, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, //
    3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 2, 2, 0, 0, 0, //
    3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 2, 2, 2, 2, 2, //
    0, 0, 19, 19, 19, 19, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, //
    0, 0, 19, 19, 19, 19, 2, 2, 4, 2, 2, 2, 2, 2, 2, 2, //
    0, 0, 19, 19, 19, 19, 0, 4, 4, 2, 2, 2, 2, 2, 2, 2, //
    0, 0, 19, 19, 19, 19, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2,
];

fn build_flags() -> Flags {
    let mut flags = Flags::new();

    for (sprite, flag) in FLAGS.into_iter().enumerate() {
        flags.fset_all(sprite, flag);
    }

    flags
}

/* SPRITE SHEET */
fn load_atlas_colors(file_name: &str) -> Vec<u8> {
    let color_map = color_map();

    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open(file_name).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];

    bytes
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            let v = chunk.iter().copied().map(|n| n as u32).collect::<Vec<_>>();

            if let &[r, g, b, _a] = &v[..] {
                let big_color = r << 16 | g << 8 | b;

                color_map[&big_color]
            } else {
                panic!("bad chunk")
            }
        })
        .collect::<Vec<Color>>()
}

fn build_sprite_sheet() -> SpriteSheet {
    let colors = load_atlas_colors("assets/atlas.png");
    let mut sprite_sheet = SpriteSheet::new();

    for (y, pixel_rows) in colors.chunks(128).into_iter().enumerate() {
        for (x, c) in pixel_rows.iter().copied().enumerate() {
            sprite_sheet.set(x, y, c);
        }
    }

    sprite_sheet
}

/// Look up from RGB color to Pico8 index
/// e.g, 0xFF004D -> 8 (Pico8 red)
fn color_map() -> HashMap<u32, Color> {
    HashMap::from_iter(
        COLORS
            .into_iter()
            .enumerate()
            .map(|(index, color)| (color, index as u8)),
    )
}
