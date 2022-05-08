use std::{collections::HashMap, fs::File};

use runty8::editor::serialize::Ppm;
use runty8::runtime::draw_context::COLORS;
use runty8::runtime::sprite_sheet::{Color, SpriteSheet};

fn main() {
    let atlas_colors = load_atlas_colors("assets/atlas.png");

    let sprite_sheet = build_sprite_sheet(&atlas_colors);

    const FILE_NAME: &str = "assets/sprite_sheet";

    Ppm::from_sprite_sheet(&sprite_sheet)
        .write_file(&format!("{}.ppm", FILE_NAME))
        .unwrap();

    std::fs::write(&format!("{}.txt", FILE_NAME), &sprite_sheet.serialize()).unwrap();
}

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

fn build_sprite_sheet(colors: &[Color]) -> SpriteSheet {
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
