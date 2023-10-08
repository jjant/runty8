use crate::{Button, InputEvent, Key, KeyState, KeyboardEvent, MouseButton, MouseEvent};

#[derive(Debug, Clone, Copy)]
pub enum InputState<T> {
    Unchanged, // None
    Changed(T),
}

#[derive(Debug)]
pub struct Input {
    pub(crate) left: InputState<KeyState>,
    pub(crate) right: InputState<KeyState>,
    pub(crate) up: InputState<KeyState>,
    pub(crate) down: InputState<KeyState>,
    pub(crate) x: InputState<KeyState>,
    pub(crate) c: InputState<KeyState>,
    pub mouse: InputState<KeyState>,
    pub mouse_position: InputState<(i32, i32)>,
}

#[allow(clippy::new_without_default)]
impl Input {
    pub fn new() -> Self {
        Self {
            left: InputState::Unchanged,
            right: InputState::Unchanged,
            up: InputState::Unchanged,
            down: InputState::Unchanged,
            x: InputState::Unchanged,
            c: InputState::Unchanged,
            mouse: InputState::Unchanged,
            mouse_position: InputState::Unchanged,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn on_event(&mut self, event: InputEvent) {
        match event {
            InputEvent::Keyboard(KeyboardEvent { key, state }) => {
                if let Some(button) = key_to_button(key) {
                    let key_ref = self.button_to_ref(button);
                    *key_ref = InputState::Changed(state);
                }
            }
            InputEvent::Mouse(MouseEvent::Button {
                button: MouseButton::Left,
                state,
            }) => {
                self.mouse = InputState::Changed(state);
            }
            InputEvent::Mouse(MouseEvent::Move { x, y }) => {
                self.mouse_position = InputState::Changed((x, y));
            }
            InputEvent::Mouse(MouseEvent::Button { .. }) => {
                // Runty8 games currently can't access other mouse buttons
            }
        }
    }

    fn button_to_ref(&mut self, button: Button) -> &mut InputState<KeyState> {
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
