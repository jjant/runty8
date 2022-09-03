use crate::font;
use crate::runtime::flags::Flags;
use crate::runtime::map::Map;
use crate::runtime::sprite_sheet::SpriteSheet;
use runty8_graphics::ReverseIf;

use super::sprite_sheet::{Color, Sprite};

const WIDTH: usize = 128;
const NUM_COMPONENTS: usize = 3;

type Buffer = [u8; NUM_COMPONENTS * WIDTH * WIDTH];
const BLACK_BUFFER: Buffer = [0; NUM_COMPONENTS * WIDTH * WIDTH];

const ORIGINAL_PALETTE: [Color; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

// Handles actually drawing to the screen buffer
#[derive(Debug)]
pub(crate) struct DrawData {
    buffer: Buffer,
    // Maybe these properties below should be in internal state?
    transparent_color: Option<Color>,
    draw_palette: [Color; 16],
    camera: (i32, i32),
}

impl DrawData {
    pub(crate) fn new() -> Self {
        DrawData {
            buffer: BLACK_BUFFER,
            transparent_color: Some(0),
            draw_palette: ORIGINAL_PALETTE,
            camera: (0, 0),
        }
    }

    pub(crate) fn append_camera(&mut self, x: i32, y: i32) {
        self.camera(self.camera.0 + x, self.camera.1 + y);
    }

    fn set_pixel_with_transparency(&mut self, index: usize, color: Color) {
        if let Some(transparent_color) = self.transparent_color {
            if color == transparent_color {
                return;
            }
        }

        self.set_pixel(index, color);
    }

    #[allow(clippy::only_used_in_recursion)]
    fn set_pixel(&mut self, index: usize, color: Color) {
        // https://pico-8.fandom.com/wiki/Pal
        let color = self.draw_palette[color as usize];
        let c = get_color(color);

        #[allow(clippy::identity_op)]
        {
            let r = ((c >> 16) & 0x0000FF) as u8;
            let g = ((c >> 8) & 0x0000FF) as u8;
            let b = ((c >> 0) & 0x0000FF) as u8;
            self.buffer[NUM_COMPONENTS * index + 0] = r;
            self.buffer[NUM_COMPONENTS * index + 1] = g;
            self.buffer[NUM_COMPONENTS * index + 2] = b;
        }
    }

    pub(crate) fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    fn print_char(&mut self, index: usize, x: i32, y: i32, color: Color) {
        let char_data = font::FONT.get(index).unwrap_or(&font::MISSING_CHAR);

        for x_offset in 0..4_i32 {
            for y_offset in 0..6 {
                let print = char_data[(x_offset + y_offset * 4) as usize] != 0;

                if print {
                    self.pset(x + x_offset, y + y_offset, color);
                }
            }
        }
    }

    fn apply_camera(&self, x: i32, y: i32) -> (i32, i32) {
        (x - self.camera.0, y - self.camera.1)
    }

    /// Returns the linear index of the pixel with (x, y) coordinates in the screen
    fn index(&self, x: i32, y: i32) -> Option<usize> {
        let x_in_bounds = 0 <= x && x < WIDTH as i32;
        let y_in_bounds = 0 <= y && y < WIDTH as i32;

        if x_in_bounds && y_in_bounds {
            Some(x as usize + y as usize * WIDTH)
        } else {
            None
        }
    }

    pub(crate) fn raw_spr(&mut self, sprite: &Sprite, x: i32, y: i32) {
        let buffer = &sprite.sprite;

        for i in 0..8 {
            for j in 0..8 {
                let x = x + i;
                let y = y + j;

                if let Some(index) = self.index(x, y) {
                    self.set_pixel_with_transparency(index, buffer[(i + j * 8) as usize]);
                }
            }
        }
    }
}

// Functions which more directly implement pico8 functionality
impl DrawData {
    pub(crate) fn pal(&mut self, c0: Color, c1: Color) {
        // https://pico-8.fandom.com/wiki/Pal
        self.draw_palette[c0 as usize] = c1;
    }

    pub(crate) fn camera(&mut self, x: i32, y: i32) {
        self.camera = (x, y);
    }

    pub(crate) fn pset(&mut self, x: i32, y: i32, color: Color) {
        let (x, y) = self.apply_camera(x, y);
        if let Some(index) = self.index(x, y) {
            self.set_pixel(index, color);
        }
    }

    pub(crate) fn rectfill(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let (x0, x1) = min_max(x0, x1);
        let (y0, y1) = min_max(y0, y1);
        let width = (x1 - x0 + 1) as u32;
        let height = (y1 - y0 + 1) as u32;

        runty8_graphics::filled_rectangle(x0, y0, width, height)
            .for_each(|(x, y)| self.pset(x, y, color))
    }

    pub(crate) fn rect(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let (x0, x1) = min_max(x0, x1);
        let (y0, y1) = min_max(y0, y1);
        let width = (x1 - x0 + 1) as u32;
        let height = (y1 - y0 + 1) as u32;

        runty8_graphics::rectangle(x0, y0, width, height).for_each(|(x, y)| self.pset(x, y, color))
    }

    pub(crate) fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        runty8_graphics::line(x0, y0, x1, y1).for_each(|(x, y)| self.pset(x, y, color));
    }

    pub(crate) fn reset_pal(&mut self) {
        self.draw_palette = ORIGINAL_PALETTE;
        // pal() resets transparency to default
        self.palt(Some(0));
    }

    pub(crate) fn palt(&mut self, transparent_color: Option<Color>) {
        self.transparent_color = transparent_color
    }

    pub(crate) fn circ(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        runty8_graphics::circle(cx, cy, radius as u32)
            .for_each(move |(x, y)| self.pset(x, y, color))
    }

    pub(crate) fn circfill(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        runty8_graphics::filled_circle(cx, cy, radius as u32)
            .for_each(move |(x, y)| self.pset(x, y, color));
    }

    pub(crate) fn print(&mut self, str: &str, x: i32, y: i32, color: Color) {
        for (pos, char) in str.chars().enumerate() {
            let index = char as u8 - font::FIRST_CHAR as u8;

            self.print_char(index as usize, x + (pos * 4) as i32, y, color);
        }
    }

    fn draw_partial_spr(
        &mut self,
        spr: &Sprite,
        x: i32,
        y: i32,
        w: f32,
        h: f32,
        flip_x: bool,
        flip_y: bool,
    ) {
        let w = crate::mid(0.0, w, 1.0);
        let h = crate::mid(0.0, h, 1.0);
        let sprite_buffer = &spr.sprite;

        let iter = runty8_graphics::Rectangle::new(
            0,
            0,
            (8.0 * w).round() as i32,
            (8.0 * h).round() as i32,
        )
        .horizontal_lines()
        .reverse_if(flip_y);

        iter.enumerate().for_each(|(j, line)| {
            let line = line.reverse_if(flip_x);

            line.enumerate().for_each(|(i, local_coords)| {
                let (local_x, local_y) = local_coords;
                let (world_x, world_y) = (x + local_x, y + local_y);
                let (screen_x, screen_y) = self.apply_camera(world_x, world_y);

                if let Some(index) = self.index(screen_x, screen_y) {
                    self.set_pixel_with_transparency(index, sprite_buffer[(i + j * 8) as usize])
                }
            });
        });
    }

    #[allow(clippy::too_many_arguments)]
    // TODO: Implement w and h params functionality
    pub fn spr_(
        &mut self,
        sprite: usize,
        sprite_sheet: &SpriteSheet,
        x: i32,
        y: i32,
        w: f32,
        h: f32,
        flip_x: bool,
        flip_y: bool,
    ) {
        let w_spr = w.ceil() as usize;
        let h_spr = h.ceil() as usize;

        let (spr_x, spr_y) = (sprite % 16, sprite / 16);
        for (w_off, h_off) in itertools::iproduct!((0..w_spr), (0..h_spr)) {
            if let Some(sprite_index) =
                SpriteSheet::sprite_index_from_coords(spr_x + w_off, spr_y + h_off)
            {
                let spr = sprite_sheet.get_sprite(sprite_index);
                self.draw_partial_spr(
                    spr,
                    x + 8 * w_off as i32,
                    y + 8 * h_off as i32,
                    w,
                    h,
                    flip_x,
                    flip_y,
                );
            }
        }
    }

    pub(crate) fn cls_color(&mut self, color: Color) {
        self.rectfill(0, 0, 127, 127, color);
    }

    /// <https://pico-8.fandom.com/wiki/Map>
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn map(
        &mut self,
        cell_x: i32,
        cell_y: i32,
        screen_x: i32,
        screen_y: i32,
        cell_w: i32,
        cell_h: i32,
        // TODO: Use layer
        layer: u8,
        map: &Map,
        flags: &Flags,
        sprite_sheet: &SpriteSheet,
    ) {
        // TODO: Handle like pico8
        for (i_x, map_x) in (cell_x..=(cell_x + cell_w)).enumerate() {
            for (i_y, map_y) in (cell_y..=(cell_y + cell_h)).enumerate() {
                let spr = map.mget(map_x, map_y);

                let flags = flags.get(spr.into()).unwrap();

                if flags & layer == layer {
                    let x = screen_x + 8 * i_x as i32;
                    let y = screen_y + 8 * i_y as i32;

                    let spr = sprite_sheet.get_sprite(spr as usize);
                    self.draw_partial_spr(spr, x, y, 1.0, 1.0, false, false);
                }
            }
        }
    }
}

impl Default for DrawData {
    fn default() -> Self {
        Self::new()
    }
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    (a.min(b), a.max(b))
}
// Pico8 api

fn get_color(index: Color) -> u32 {
    COLORS[index as usize]
}

// Add _FF at the end for alpha
pub const COLORS: [u32; 16] = [
    0x000000, //
    0x1D2B53, //
    0x7E2553, //
    0x008751, //
    0xAB5236, //
    0x5F574F, //
    0xC2C3C7, //
    0xFFF1E8, //
    0xFF004D, //
    0xFFA300, //
    0xFFEC27, //
    0x00E436, //
    0x29ADFF, //
    0x83769C, //
    0xFF77A8, //
    0xFFCCAA, //
];

/// Named constants for the colors in the Pico8 palette.
pub mod colors {
    use crate::runtime::sprite_sheet::Color;

    pub const BLACK: Color = 0;
    pub const DARK_BLUE: Color = 1;
    pub const DARK_PURPLE: Color = 2;
    pub const DARK_GREEN: Color = 3;
    pub const BROWN: Color = 4;
    pub const DARK_GREY: Color = 5;
    pub const LIGHT_GREY: Color = 6;
    pub const WHITE: Color = 7;
    pub const RED: Color = 8;
    pub const ORANGE: Color = 9;
    pub const YELLOW: Color = 10;
    pub const GREEN: Color = 11;
    pub const BLUE: Color = 12;
    pub const LAVENDER: Color = 13;
    pub const PINK: Color = 14;
    pub const LIGHT_PEACH: Color = 15;
}
