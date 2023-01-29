use crate::{Button, InputEvent, Key, KeyState, KeyboardEvent, MouseButton, MouseEvent};

#[derive(Debug)]
pub struct Input {
    pub(crate) left: Option<bool>,
    pub(crate) right: Option<bool>,
    pub(crate) up: Option<bool>,
    pub(crate) down: Option<bool>,
    pub(crate) x: Option<bool>,
    pub(crate) c: Option<bool>,
    pub mouse: Option<bool>,
    pub mouse_x: i32,
    pub mouse_y: i32,
}

#[allow(clippy::new_without_default)]
impl Input {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            up: None,
            down: None,
            x: None,
            c: None,
            mouse: None,
            // TODO: Initialize mouse properly
            mouse_x: 64,
            mouse_y: 64,
        }
    }

    pub fn on_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::Keyboard(KeyboardEvent { key, state }) => {
                if let Some(button) = key_to_button(key) {
                    let key_ref = self.button_to_ref(button);
                    *key_ref = Some(state == KeyState::Down);
                }
            }
            InputEvent::Mouse(MouseEvent::Button {
                button: MouseButton::Left,
                state,
            }) => {
                self.mouse = Some(state == KeyState::Down);
            }
            InputEvent::Mouse(MouseEvent::Move { x, y }) => {
                self.mouse_x = x;
                self.mouse_y = y;
            }
            InputEvent::Mouse(MouseEvent::Button { .. }) => {
                // Runty8 games currently can't access other mouse buttons
            }
        }
    }

    fn button_to_ref(&mut self, button: Button) -> &mut Option<bool> {
        match button {
            Button::Cross => &mut self.x,
            Button::Circle => &mut self.c,
            Button::Left => &mut self.left,
            Button::Right => &mut self.right,
            Button::Up => &mut self.up,
            Button::Down => &mut self.down,
            Button::Mouse => &mut self.mouse,
        }
    }
}

fn key_to_button(key: Key) -> Option<Button> {
    match key {
        Key::X => Some(Button::Cross),
        Key::C => Some(Button::Circle),
        Key::LeftArrow => Some(Button::Left),
        Key::RightArrow => Some(Button::Right),
        Key::UpArrow => Some(Button::Up),
        Key::DownArrow => Some(Button::Down),
        _ => None,
    }
}
