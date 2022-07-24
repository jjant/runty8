use crate::runtime::input::Keys;
use ButtonState::*;

#[derive(Debug)]
pub(crate) struct State {
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

impl State {
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

    pub(crate) fn button(&self, button: Button) -> &ButtonState {
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

    pub(crate) fn mouse(&self) -> (i32, i32) {
        (self.mouse_x, self.mouse_y)
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

    pub(crate) fn btn(&self) -> bool {
        match *self {
            JustPressed => true,
            Held => true,
            NotPressed => false,
        }
    }

    pub(crate) fn btnp(&self) -> bool {
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
