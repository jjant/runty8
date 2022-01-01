pub mod app;
mod font;
mod screen;

pub use app::App;

const WIDTH: usize = 128;
const NUM_COMPONENTS: usize = 3;
type Buffer = [u8; NUM_COMPONENTS * WIDTH * WIDTH];
const BLACK_BUFFER: Buffer = [0; NUM_COMPONENTS * WIDTH * WIDTH];

pub struct DrawContext {
    buffer: Buffer,
}

// Add _FF at the end for alpha
const COLORS: [u32; 16] = [
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

pub type Color = u8; // Actually a u4
type Sprite = [Color; 8 * 8];

impl DrawContext {
    fn new() -> Self {
        Self {
            buffer: BLACK_BUFFER,
        }
    }

    pub fn cls(&mut self) {
        self.buffer = BLACK_BUFFER;
    }

    pub fn line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Color) {
        // Uses Bresenham's algorithm, last snippet in this article
        // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy; /* error value e_xy */

        loop {
            self.pset(x0, y0, color);
            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                /* e_xy+e_x > 0 */
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                /* e_xy+e_y < 0 */
                err += dx;
                y0 += sy;
            }
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

    pub fn raw_spr(&mut self, sprite: Sprite, x: i32, y: i32) {
        for i in 0..8 {
            for j in 0..8 {
                self.pset(x + i, y + j, sprite[(i + j * 8) as usize]);
            }
        }
    }

    pub fn print(&mut self, str: &str, x: i32, y: i32, color: Color) {
        for (pos, char) in str.chars().enumerate() {
            let index = char as u8 - font::FIRST_CHAR as u8;

            self.print_char(index as usize, x + (pos * 4) as i32, y, color);
        }
    }

    fn print_char(&mut self, index: usize, x: i32, y: i32, color: Color) {
        let char_data = font::FONT[index];

        for x_offset in 0..4_i32 {
            for y_offset in 0..6 {
                let print = char_data[(x_offset + y_offset * 4) as usize] != 0;

                if print {
                    self.pset(x + x_offset, y + y_offset, color);
                }
            }
        }
    }

    // pub fn spr(&mut self, n: u8, x: usize, y: usize) {
    //     let sprite = self.get_sprite(n);

    //     for i in 0..8 {
    //         for j in 0..8 {
    //             self.pset(x, y, sprite[i + j * 8]);
    //         }
    //     }
    // }

    pub fn pset(&mut self, x: i32, y: i32, color: Color) {
        if let Some(index) = self.index(x, y) {
            let c = get_color(color);
            let r = ((c >> 16) & 0x0000FF) as u8;
            let g = ((c >> 8) & 0x0000FF) as u8;
            let b = ((c >> 0) & 0x0000FF) as u8;

            self.buffer[NUM_COMPONENTS * index + 0] = r;
            self.buffer[NUM_COMPONENTS * index + 1] = g;
            self.buffer[NUM_COMPONENTS * index + 2] = b;
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
}

fn get_color(index: Color) -> u32 {
    COLORS[index as usize]
}

#[derive(Debug)]
pub struct State {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    x: bool,
    c: bool,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_pressed: bool,
}

impl State {
    fn new() -> Self {
        Self {
            left: false,
            right: false,
            up: false,
            down: false,
            x: false,
            c: false,
            mouse_x: 64,
            mouse_y: 64,
            mouse_pressed: false,
        }
    }

    pub fn btn(&self, button: Button) -> bool {
        match button {
            Button::Left => self.left,
            Button::Right => self.right,
            Button::Up => self.up,
            Button::Down => self.down,
            Button::X => self.x,
            Button::C => self.c,
        }
    }
}

pub enum Button {
    Left,
    Right,
    Up,
    Down,
    X,
    C,
}

pub fn run_app<T: App + 'static>() {
    let state = State::new();
    let draw_context = DrawContext::new();

    screen::do_something::<T>(state, draw_context);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
