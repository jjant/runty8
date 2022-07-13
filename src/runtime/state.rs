use super::{flags::Flags, input::Keys, map::Map, sprite_sheet::SpriteSheet};
use crate::Resources;
use ButtonState::*;

#[derive(Debug)]
pub struct State<'a> {
    pub(crate) internal_state: &'a InternalState,
    pub(crate) sprite_sheet: &'a mut SpriteSheet,
    pub(crate) sprite_flags: &'a mut Flags,
    pub(crate) map: &'a mut Map,
}

#[derive(Debug)]
pub(crate) struct InternalState {
    left: ButtonState,
    right: ButtonState,
    up: ButtonState,
    down: ButtonState,
    x: ButtonState,
    c: ButtonState,
    pub mouse_x: i32,
    pub mouse_y: i32,
    mouse_pressed: ButtonState,
}

impl InternalState {
    pub(crate) fn new() -> Self {
        Self {
            left: NotPressed,
            right: NotPressed,
            up: NotPressed,
            down: NotPressed,
            x: NotPressed,
            c: NotPressed,
            mouse_x: 64,
            mouse_y: 64,
            mouse_pressed: NotPressed,
        }
    }

    pub(crate) fn on_mouse_move(&mut self, mouse_x: i32, mouse_y: i32) {
        self.mouse_x = mouse_x;
        self.mouse_y = mouse_y;
    }

    pub(crate) fn update_keys(&mut self, keys: &Keys) {
        self.left.update(keys.left);
        self.right.update(keys.right);
        self.up.update(keys.up);
        self.down.update(keys.down);
        self.x.update(keys.x);
        self.c.update(keys.c);
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

// Temporary?
impl<'a> State<'a> {
    pub fn mouse(&self) -> (i32, i32) {
        (self.internal_state.mouse_x, self.internal_state.mouse_y)
    }
}

impl<'a> State<'a> {
    pub(crate) fn new(internal_state: &'a InternalState, resources: &'a mut Resources) -> Self {
        let Resources {
            sprite_sheet,
            sprite_flags,
            map,
            ..
        } = resources;

        Self {
            internal_state,
            sprite_sheet,
            sprite_flags,
            map,
        }
    }

    pub fn mget(&self, cel_x: i32, cel_y: i32) -> u8 {
        self.map.mget(cel_x, cel_y)
    }

    pub fn mset(&mut self, cel_x: usize, cel_y: usize, sprite: u8) {
        self.map.mset(cel_x, cel_y, sprite);
    }

    pub fn fget(&self, sprite: usize) -> u8 {
        self.sprite_flags.get(sprite).unwrap()
    }

    // TODO: Check we do the same left-to-right (or vice versa)
    // order as pico8
    pub fn fget_n(&self, sprite: usize, flag: u8) -> bool {
        self.sprite_flags.fget_n(sprite, flag)
    }

    pub fn fset(&mut self, sprite: usize, flag: usize, value: bool) -> u8 {
        self.sprite_flags.fset(sprite, flag, value)
    }

    pub fn btn(&self, button: Button) -> bool {
        self.internal_state.button(button).btn()
    }

    pub fn btnp(&self, button: Button) -> bool {
        self.internal_state.button(button).btnp()
    }
}

#[derive(Debug)]
pub(crate) enum ButtonState {
    JustPressed, // btn => true, btnp => true
    Held,        // btn => true, btnp => false
    NotPressed,
}

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
        matches!(*self, JustPressed)
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
