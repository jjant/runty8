pub mod app;
mod draw;
mod editor;
mod font;
mod screen;

use std::{fs::File, io::Read};

pub use app::App;
use itertools::Itertools;

const WIDTH: usize = 128;
const NUM_COMPONENTS: usize = 3;
type Buffer = [u8; NUM_COMPONENTS * WIDTH * WIDTH];
const BLACK_BUFFER: Buffer = [0; NUM_COMPONENTS * WIDTH * WIDTH];

#[repr(transparent)]
pub struct Sprite {
    pub sprite: [Color],
}

impl Sprite {
    fn new(sprite: &[u8]) -> &Self {
        unsafe { &*(sprite as *const [u8] as *const Self) }
    }

    fn new_mut(sprite: &mut [u8]) -> &mut Self {
        unsafe { &mut *(sprite as *mut [u8] as *mut Self) }
    }

    pub fn pset(&mut self, x: isize, y: isize, color: Color) {
        self.sprite[Self::index(x, y).unwrap()] = color;
    }

    pub fn pget(&self, x: isize, y: isize) -> Color {
        self.sprite[Self::index(x, y).unwrap()]
    }

    fn index(x: isize, y: isize) -> Option<usize> {
        (x + y * (SPRITE_WIDTH as isize)).try_into().ok()
    }

    pub(crate) fn shift_up(&mut self) {
        self.sprite.rotate_left(8);
    }

    pub(crate) fn shift_down(&mut self) {
        self.sprite.rotate_right(8);
    }

    pub(crate) fn shift_left(&mut self) {
        self.sprite
            .chunks_mut(SPRITE_WIDTH)
            .for_each(|row| row.rotate_left(1));
    }

    pub(crate) fn shift_right(&mut self) {
        self.sprite
            .chunks_mut(SPRITE_WIDTH)
            .for_each(|row| row.rotate_right(1));
    }
}

#[derive(Debug)]
pub(crate) struct SpriteSheet {
    sprite_sheet: Vec<Color>,
}

pub const SPRITE_WIDTH: usize = 8;
pub const SPRITE_HEIGHT: usize = 8;

impl SpriteSheet {
    pub const SPRITE_COUNT: usize = 256;

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            sprite_sheet: vec![0; Self::SPRITE_COUNT * SPRITE_WIDTH * SPRITE_HEIGHT],
        }
    }

    pub fn get_sprite(&self, sprite: usize) -> &Sprite {
        let index = self.sprite_index(sprite);

        Sprite::new(&self.sprite_sheet[index..(index + SPRITE_WIDTH * SPRITE_HEIGHT)])
    }

    pub(crate) fn get_sprite_mut(&mut self, sprite: usize) -> &mut Sprite {
        let index = self.sprite_index(sprite);

        Sprite::new_mut(&mut self.sprite_sheet[index..(index + SPRITE_WIDTH * SPRITE_HEIGHT)])
    }

    fn sprite_index(&self, sprite: usize) -> usize {
        // How many pixels we need to skip to get to the start of this sprite.
        sprite * SPRITE_WIDTH * SPRITE_HEIGHT
    }

    pub fn serialize(&self) -> String {
        let lines = self.sprite_sheet.chunks(128).map(|chunk| {
            Itertools::intersperse(chunk.iter().map(|n| format!("{:X}", n)), "".to_owned())
                .collect()
        });

        Itertools::intersperse(lines, "\n".to_owned()).collect::<String>()
    }

    pub fn deserialize(str: &str) -> Self {
        let sprite_sheet = str
            .as_bytes()
            .into_iter()
            .copied()
            .filter_map(|c| (c as char).to_digit(16))
            .map(|c| c as u8)
            .collect();

        Self { sprite_sheet }
    }
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

// TODO: Make a more reliable version of this.
// TODO: Improve capacity calculation? It's kinda flakey
fn deserialize() -> SpriteSheet {
    let capacity = 128 * 128 + 128;
    let mut file = String::with_capacity(capacity);
    File::open("sprite_sheet.txt")
        .expect("Couldn't OPEN file")
        .read_to_string(&mut file)
        .expect("Couldn't READ file");

    SpriteSheet::deserialize(&file)
}

pub struct DrawContext {
    buffer: Buffer,
    state: State,
    transparent_color: Option<Color>,
}

impl DrawContext {
    fn new(state: State) -> Self {
        Self {
            buffer: BLACK_BUFFER,
            state,
            transparent_color: Some(0),
        }
    }

    pub fn cls(&mut self) {
        self.buffer = BLACK_BUFFER;
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        for (x, y) in draw::line(x0, y0, x1, y1) {
            self.pset(x, y, color);
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

    pub fn spr(&mut self, sprite: usize, x: i32, y: i32) {
        let sprite = self.state.sprite_sheet.get_sprite(sprite);
        let buffer = &sprite.sprite;

        for i in 0..8 {
            for j in 0..8 {
                if let Some(index) = self.index(x + i, y + j) {
                    Self::set_pixel(
                        &mut self.buffer,
                        self.transparent_color,
                        index,
                        buffer[(i + j * 8) as usize],
                    )
                }
            }
        }
    }

    pub fn raw_spr(&mut self, sprite: &Sprite, x: i32, y: i32) {
        let buffer = &sprite.sprite;

        for i in 0..8 {
            for j in 0..8 {
                self.pset(x + i, y + j, buffer[(i + j * 8) as usize]);
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

    fn set_pixel(
        buffer: &mut [Color],
        transparent_color: Option<Color>,
        index: usize,
        color: Color,
    ) {
        let c = get_color(color);

        if let Some(transparent_color) = transparent_color {
            if color == transparent_color {
                return;
            }
        }

        let r = ((c >> 16) & 0x0000FF) as u8;
        let g = ((c >> 8) & 0x0000FF) as u8;
        let b = ((c >> 0) & 0x0000FF) as u8;

        buffer[NUM_COMPONENTS * index + 0] = r;
        buffer[NUM_COMPONENTS * index + 1] = g;
        buffer[NUM_COMPONENTS * index + 2] = b;
    }

    pub fn pset(&mut self, x: i32, y: i32, color: Color) {
        if let Some(index) = self.index(x, y) {
            Self::set_pixel(&mut self.buffer, self.transparent_color, index, color);
        }
    }

    pub fn palt(&mut self, transparent_color: Option<Color>) {
        self.transparent_color = transparent_color
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
pub enum Scene {
    Editor,
    App,
}

impl Scene {
    fn initial() -> Self {
        Scene::Editor
    }

    pub fn flip(&mut self) {
        *self = match self {
            Scene::Editor => Scene::App,
            Scene::App => Scene::Editor,
        }
    }
}

#[derive(Debug)]
pub struct State {
    left: ButtonState,
    right: ButtonState,
    up: ButtonState,
    down: ButtonState,
    x: ButtonState,
    c: ButtonState,
    pub(crate) escape: ButtonState,
    pub mouse_x: i32,
    pub mouse_y: i32,
    mouse_pressed: ButtonState,
    pub(crate) scene: Scene,
    pub(crate) sprite_sheet: SpriteSheet,
}

impl State {
    fn new() -> Self {
        let sprite_sheet = deserialize();

        Self {
            left: NotPressed,
            right: NotPressed,
            up: NotPressed,
            down: NotPressed,
            x: NotPressed,
            c: NotPressed,
            escape: NotPressed,
            mouse_x: 64,
            mouse_y: 64,
            mouse_pressed: NotPressed,
            scene: Scene::initial(),
            sprite_sheet,
        }
    }

    pub fn btn(&self, button: Button) -> bool {
        self.button(button).btn()
    }

    pub fn btnp(&self, button: Button) -> bool {
        self.button(button).btnp()
    }

    pub(crate) fn update_keys(&mut self, keys: &Keys) {
        self.left.update(keys.left);
        self.right.update(keys.right);
        self.up.update(keys.up);
        self.down.update(keys.down);
        self.x.update(keys.x);
        self.c.update(keys.c);
        self.escape.update(keys.escape);
        self.mouse_pressed.update(keys.mouse);
    }

    fn button(&self, button: Button) -> &ButtonState {
        match button {
            Button::Left => &self.left,
            Button::Right => &self.right,
            Button::Up => &self.up,
            Button::Down => &self.down,
            Button::X => &self.x,
            Button::C => &self.c,
            Button::Mouse => &self.mouse_pressed,
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
    Mouse,
}

pub fn run_app<T: App + 'static>() {
    let state = State::new();
    let draw_context = DrawContext::new(state);

    screen::do_something::<T>(draw_context);
}

// TODO: Implement properly
// TODO2: I think this is fine, now?
#[derive(Debug)]
pub(crate) enum ButtonState {
    JustPressed,
    Held,
    NotPressed,
}

use screen::Keys;
use ButtonState::*;

impl ButtonState {
    fn update(&mut self, is_pressed: Option<bool>) {
        match is_pressed {
            Some(is_pressed) => {
                if is_pressed {
                    self.press()
                } else {
                    self.unpress()
                }
            }
            None => self.no_change(),
        }
    }

    // A frame has passed but we've registered no event related to this key.
    fn no_change(&mut self) {
        *self = match self {
            JustPressed => Held,
            Held => Held,
            NotPressed => NotPressed,
        }
    }

    // Caution: This may come either from a "first" press or a "repeated" press.
    fn press(&mut self) {
        *self = match self {
            JustPressed => Held,
            Held => Held,
            NotPressed => JustPressed,
        }
    }

    fn unpress(&mut self) {
        *self = NotPressed;
    }

    pub fn btn(&self) -> bool {
        match *self {
            JustPressed => true,
            Held => true,
            NotPressed => false,
        }
    }

    pub fn btnp(&self) -> bool {
        match *self {
            JustPressed => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
