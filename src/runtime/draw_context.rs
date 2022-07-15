use crate::{draw, font};

use super::sprite_sheet::{Color, Sprite};
use super::state::{Button, State};

const WIDTH: usize = 128;
const NUM_COMPONENTS: usize = 3;
type Buffer = [u8; NUM_COMPONENTS * WIDTH * WIDTH];
const BLACK_BUFFER: Buffer = [0; NUM_COMPONENTS * WIDTH * WIDTH];

pub struct DrawData {
    pub(crate) buffer: Buffer,
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
}

impl Default for DrawData {
    fn default() -> Self {
        Self::new()
    }
}

/// Context providing pico8 functionality:
/// drawing to the screen, querying IO devices (button states), etc.
// TODO: Rename to someting like Runty8?
pub struct DrawContext<'a, 'resources> {
    data: &'a mut DrawData,
    // TODO: make pub(crate)
    pub(crate) state: &'a mut State<'resources>,
}

const ORIGINAL_PALETTE: [Color; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

impl<'a, 'resources> DrawContext<'a, 'resources> {
    pub(crate) fn new(state: &'a mut State<'resources>, data: &'a mut DrawData) -> Self {
        Self { state, data }
    }

    pub(crate) fn append_camera(&mut self, x: i32, y: i32) {
        self.camera(self.data.camera.0 + x, self.data.camera.1 + y);
    }

    pub(crate) fn raw_spr(&mut self, sprite: &Sprite, x: i32, y: i32) {
        let buffer = &sprite.sprite;

        for i in 0..8 {
            for j in 0..8 {
                let x = x + i;
                let y = y + j;

                if let Some(index) = self.index(x, y) {
                    self.data
                        .set_pixel_with_transparency(index, buffer[(i + j * 8) as usize]);
                }
            }
        }
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

    fn index(&self, x: i32, y: i32) -> Option<usize> {
        let x_in_bounds = 0 <= x && x < WIDTH as i32;
        let y_in_bounds = 0 <= y && y < WIDTH as i32;

        if x_in_bounds && y_in_bounds {
            Some(x as usize + y as usize * WIDTH)
        } else {
            None
        }
    }

    fn apply_camera(&self, x: i32, y: i32) -> (i32, i32) {
        (x - self.data.camera.0, y - self.data.camera.1)
    }
}

// Pico8 api
impl<'a, 'resources> DrawContext<'a, 'resources> {
    pub fn btn(&self, button: Button) -> bool {
        self.state.btn(button)
    }

    #[allow(clippy::too_many_arguments)]
    // TODO: Implement w and h params functionality
    pub fn spr_(
        &mut self,
        sprite: usize,
        x: i32,
        y: i32,
        _w: f32,
        _h: f32,
        flip_x: bool,
        flip_y: bool,
    ) {
        let sprite = self.state.sprite_sheet.get_sprite(sprite);
        let buffer = &sprite.sprite;

        for i in 0..8 {
            for j in 0..8 {
                let world_x = if flip_x { x + 7 - i } else { x + i };
                let world_y = if flip_y { y + 7 - j } else { y + j };

                let (x, y) = self.apply_camera(world_x, world_y);
                if let Some(index) = self.index(x, y) {
                    self.data
                        .set_pixel_with_transparency(index, buffer[(i + j * 8) as usize])
                }
            }
        }
    }

    pub fn spr(&mut self, sprite: usize, x: i32, y: i32) {
        self.spr_(sprite, x, y, 1.0, 1.0, false, false)
    }

    pub fn print(&mut self, str: &str, x: i32, y: i32, color: Color) {
        for (pos, char) in str.chars().enumerate() {
            let index = char as u8 - font::FIRST_CHAR as u8;

            self.print_char(index as usize, x + (pos * 4) as i32, y, color);
        }
    }

    pub fn fget(&self, sprite: usize) -> u8 {
        self.state.fget(sprite)
    }

    pub fn fget_n(&self, sprite: usize, flag: u8) -> bool {
        self.state.fget_n(sprite, flag)
    }

    pub fn reset_pal(&mut self) {
        self.data.draw_palette = ORIGINAL_PALETTE;
        // pal() resets transparency to default
        self.palt(Some(0));
    }

    pub fn pal(&mut self, c0: Color, c1: Color) {
        // https://pico-8.fandom.com/wiki/Pal
        self.data.draw_palette[c0 as usize] = c1;
    }

    pub fn pset(&mut self, x: i32, y: i32, color: Color) {
        let (x, y) = self.apply_camera(x, y);
        if let Some(index) = self.index(x, y) {
            self.data.set_pixel(index, color);
        }
    }

    pub fn palt(&mut self, transparent_color: Option<Color>) {
        self.data.transparent_color = transparent_color
    }

    pub fn camera(&mut self, x: i32, y: i32) {
        self.data.camera = (x, y);
    }

    pub fn cls(&mut self) {
        self.cls_color(colors::BLACK);
    }

    pub fn cls_color(&mut self, color: Color) {
        self.rectfill(0, 0, 127, 127, color);
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        for (x, y) in draw::line(x0, y0, x1, y1) {
            self.pset(x, y, color);
        }
    }

    // Taken from Pemsa, a C++ implementation of pico8.
    // This looks similar to Bresenham's circle drawing algorithm.
    //
    // See: https://github.com/egordorichev/pemsa/blob/master/src/pemsa/graphics/pemsa_graphics_api.cpp#L393
    pub fn circ(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        fn plot(this: &mut DrawContext, cx: i32, cy: i32, x: i32, y: i32, c: u8) {
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

    // Taken from Pemsa, a C++ implementation of pico8.
    // This looks similar to Bresenham's circle drawing algorithm.
    //
    // See: https://github.com/egordorichev/pemsa/blob/master/src/pemsa/graphics/pemsa_graphics_api.cpp#L393
    pub fn circfill(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        fn plot(this: &mut DrawContext, cx: i32, cy: i32, x: i32, y: i32, c: u8) {
            this.line(cx - x, cy + y, cx + x, cy + y, c);

            if y != 0 {
                this.line(cx - x, cy - y, cx + x, cy - y, c);
            }
        }

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

    pub fn rectfill(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        for y in y0..=y1 {
            self.line(x0, y, x1, y, color);
        }
    }

    pub fn rect(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        self.line(x0, y0, x1, y0, color);
        self.line(x0, y0, x0, y1, color);
        self.line(x0, y1, x1, y1, color);
        self.line(x1, y0, x1, y1, color);
    }

    /// <https://pico-8.fandom.com/wiki/Map>
    #[allow(clippy::too_many_arguments)]
    pub fn map(
        &mut self,
        cell_x: i32,
        cell_y: i32,
        screen_x: i32,
        screen_y: i32,
        cell_w: i32,
        cell_h: i32,
        // TODO: Use layer
        layer: u8,
    ) {
        // TODO: Handle like pico8

        for (i_x, map_x) in (cell_x..=(cell_x + cell_w)).enumerate() {
            for (i_y, map_y) in (cell_y..=(cell_y + cell_h)).enumerate() {
                let spr = self.mget(map_x, map_y);

                let flags = self.fget(spr.into());

                if flags & layer == layer {
                    let x = screen_x + 8 * i_x as i32;
                    let y = screen_y + 8 * i_y as i32;

                    self.spr(spr as usize, x, y);
                }
            }
        }
    }

    pub fn mget(&self, x: i32, y: i32) -> u8 {
        self.state.mget(x, y)
    }

    pub fn mouse(&self) -> (i32, i32) {
        self.state.mouse()
    }
}

fn get_color(index: Color) -> u32 {
    COLORS[index as usize]
}

// Add _FF at the end for alpha
pub const COLORS: [u32; 16] = [
    0x000000, // _FF,
    0x1D2B53, // _FF,
    0x7E2553, // _FF,
    0x008751, // _FF,
    0xAB5236, // _FF,
    0x5F574F, // _FF,
    0xC2C3C7, // _FF,
    0xFFF1E8, // _FF,
    0xFF004D, // _FF,
    0xFFA300, // _FF,
    0xFFEC27, // _FF,
    0x00E436, // _FF,
    0x29ADFF, // _FF,
    0x83769C, // _FF,
    0xFF77A8, // _FF,
    0xFFCCAA, // _FF,
];

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
