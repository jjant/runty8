pub mod app;
mod screen;

use app::App;

const WIDTH: usize = 128;
const NUM_COMPONENTS: usize = 3;
type Buffer = [u8; NUM_COMPONENTS * WIDTH * WIDTH];
const BLACK_BUFFER: Buffer = [0; NUM_COMPONENTS * WIDTH * WIDTH];

pub struct DrawContext {
    buffer: Buffer,
}

const COLORS: [u32; 16] = [
    0x000000, 0x1D2B53, 0x7E2553, 0x008751, 0xAB5236, 0x5F574F, 0xC2C3C7, 0xFFF1E8, 0xFF004D,
    0xFFA300, 0xFFEC27, 0x00E436, 0x29ADFF, 0x83769C, 0xFF77A8, 0xFFCCAA,
];

impl DrawContext {
    fn new() -> Self {
        Self {
            buffer: BLACK_BUFFER,
        }
    }

    pub fn cls(&mut self) {
        self.buffer = BLACK_BUFFER;
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: [u8; NUM_COMPONENTS]) {
        // TODO: Use a proper algorithm (this breaks with vertical lines);
        let dy = y1 - y0;
        let dx = x1 - x0;
        let slope = dy as f32 / dx as f32;

        let mut y = y0 as f32;
        for x in x0..=x1 {
            self.set_color(x as usize, y.floor() as usize, color);
            y += slope;
        }
    }

    pub fn rectfill(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: [u8; NUM_COMPONENTS]) {
        for y in y0..=y1 {
            self.line(x0, y, x1, y, color);
        }
    }

    fn set_color(&mut self, x: usize, y: usize, color: [u8; NUM_COMPONENTS]) {
        for i in 0..NUM_COMPONENTS {
            if let Some(index) = self.index(x, y) {
                self.buffer[NUM_COMPONENTS * index + i] = color[i];
            }
        }
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        #![allow(unused_comparisons)]
        // TODO: Maybe change to isize
        let x_in_bounds = 0 <= x && x < WIDTH;
        let y_in_bounds = 0 <= y && y < WIDTH;

        if x_in_bounds && y_in_bounds {
            Some(x + y * WIDTH)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct State {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    x: bool,
    c: bool,
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
