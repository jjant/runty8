use crate::flags::Flags;
use crate::map::Map;
use crate::sprite_sheet::SpriteSheet;
use crate::util::{min_max, MinMax};
use crate::Color;
use crate::{draw, font};

use crate::sprite_sheet::Sprite;

const WIDTH: usize = 128;
const NUM_COMPONENTS: usize = 3;

type Buffer = [u8; NUM_COMPONENTS * WIDTH * WIDTH];
const BLACK_BUFFER: Buffer = [0; NUM_COMPONENTS * WIDTH * WIDTH];

const ORIGINAL_PALETTE: [Color; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

// Handles actually drawing to the screen buffer
#[derive(Debug)]
pub struct DrawData {
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

    pub fn buffer(&self) -> &Buffer {
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

    pub(crate) fn quarter_bresenham(
        &mut self,
        cx: i32,
        cy: i32,
        radius: i32,
        color: Color,
        plot: fn(&mut Self, i32, i32, i32, i32, Color),
    ) {
        let mut x = radius;
        let mut y = 0;
        let mut error = 1 - radius;

        while y <= x {
            plot(self, cx, cy, x, y, color);

            if error < 0 {
                error += 2 * y + 3;
            } else {
                if x != y {
                    plot(self, cx, cy, y, x, color);
                }

                x -= 1;
                error += 2 * (y - x) + 3;
            }

            y += 1;
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
        let MinMax { min: y0, max: y1 } = min_max(y0, y1);
        for y in y0..=y1 {
            self.line(x0, y, x1, y, color);
        }
    }

    pub(crate) fn rect(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        self.line(x0, y0, x1, y0, color);
        self.line(x0, y0, x0, y1, color);
        self.line(x0, y1, x1, y1, color);
        self.line(x1, y0, x1, y1, color);
    }

    pub(crate) fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        for (x, y) in draw::line(x0, y0, x1, y1) {
            self.pset(x, y, color);
        }
    }

    pub(crate) fn reset_pal(&mut self) {
        self.draw_palette = ORIGINAL_PALETTE;
        // pal() resets transparency to default
        self.palt(Some(0));
    }

    pub(crate) fn palt(&mut self, transparent_color: Option<Color>) {
        self.transparent_color = transparent_color
    }

    // Taken from Pemsa, a C++ implementation of pico8.
    // This looks similar to Bresenham's circle drawing algorithm.
    //
    // See: https://github.com/egordorichev/pemsa/blob/master/src/pemsa/graphics/pemsa_graphics_api.cpp#L393
    pub(crate) fn circ(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        fn plot(this: &mut DrawData, cx: i32, cy: i32, x: i32, y: i32, c: u8) {
            let points = [
                (x, y),
                (-x, y),
                (x, -y),
                (-x, -y),
                (y, x),
                (-y, x),
                (y, -x),
                (-y, -x),
            ];

            for (x, y) in points {
                this.pset(cx + x, cy + y, c);
            }
        }

        self.quarter_bresenham(cx, cy, radius, color, plot);
    }

    // Taken from Pemsa, a C++ implementation of pico8.
    // This looks similar to Bresenham's circle drawing algorithm.
    //
    // See: https://github.com/egordorichev/pemsa/blob/master/src/pemsa/graphics/pemsa_graphics_api.cpp#L393
    pub(crate) fn circfill(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        fn plot(this: &mut DrawData, cx: i32, cy: i32, x: i32, y: i32, c: u8) {
            this.line(cx - x, cy + y, cx + x, cy + y, c);

            if y != 0 {
                this.line(cx - x, cy - y, cx + x, cy - y, c);
            }
        }

        self.quarter_bresenham(cx, cy, radius, color, plot);
    }

    pub(crate) fn print(&mut self, str: &str, x: i32, y: i32, color: Color) {
        for (pos, char) in str.chars().enumerate() {
            let index = char as u8 - font::FIRST_CHAR as u8;

            self.print_char(index as usize, x + (pos * 4) as i32, y, color);
        }
    }

    #[allow(clippy::too_many_arguments)]
    // TODO: Implement w and h params functionality
    pub fn spr_(
        &mut self,
        sprite: &Sprite,
        x: i32,
        y: i32,
        _w: f32,
        _h: f32,
        flip_x: bool,
        flip_y: bool,
    ) {
        let buffer = &sprite.sprite;

        for i in 0..8 {
            for j in 0..8 {
                let world_x = if flip_x { x + 7 - i } else { x + i };
                let world_y = if flip_y { y + 7 - j } else { y + j };

                let (x, y) = self.apply_camera(world_x, world_y);
                if let Some(index) = self.index(x, y) {
                    self.set_pixel_with_transparency(index, buffer[(i + j * 8) as usize])
                }
            }
        }
    }

    pub(crate) fn spr(&mut self, sprite: &Sprite, x: i32, y: i32) {
        self.spr_(sprite, x, y, 1.0, 1.0, false, false)
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
                    self.spr(spr, x, y);
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

// Pico8 api

fn get_color(index: Color) -> u32 {
    COLORS[index as usize]
}

// Add _FF at the end for alpha
pub const COLORS: [u32; 16] = [
    0x000000, // Black
    0x1D2B53, // Dark blue
    0x7E2553, // Dark purple
    0x008751, // Dark green
    0xAB5236, // Brown
    0x5F574F, // Dark grey
    0xC2C3C7, // Light grey
    0xFFF1E8, // White
    0xFF004D, // Red
    0xFFA300, // Orange
    0xFFEC27, // Yellow
    0x00E436, // Green
    0x29ADFF, // Blue
    0x83769C, // Lavender
    0xFF77A8, // Pink
    0xFFCCAA, // Light peach
];

/// Named constants for the colors in the Pico8 palette.
pub mod colors {
    use crate::Color;

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
