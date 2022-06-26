use crate::{Key, KeyState, KeyboardEvent};

#[derive(Debug)]
pub(crate) struct Keys {
    pub(crate) left: Option<bool>,
    pub(crate) right: Option<bool>,
    pub(crate) up: Option<bool>,
    pub(crate) down: Option<bool>,
    pub(crate) x: Option<bool>,
    pub(crate) c: Option<bool>,
    pub(crate) escape: Option<bool>,
    pub(crate) mouse: Option<bool>,
}

impl Keys {
    pub(crate) fn new() -> Self {
        Self {
            left: None,
            right: None,
            up: None,
            down: None,
            x: None,
            c: None,
            escape: None,
            mouse: None,
        }
    }

    pub(crate) fn on_event(&mut self, event: KeyboardEvent) {
        let mut other = None;
        let key_ref = match event.key {
            Key::X => &mut self.x,
            Key::C => &mut self.c,
            Key::LeftArrow => &mut self.left,
            Key::UpArrow => &mut self.up,
            Key::RightArrow => &mut self.right,
            Key::DownArrow => &mut self.down,
            Key::Escape => &mut self.escape,
            _ => &mut other,
        };
        *key_ref = Some(event.state == KeyState::Down);
    }
}
