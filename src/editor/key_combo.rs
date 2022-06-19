use std::collections::{hash_map::Entry, HashMap};

use crate::{Key, KeyState};

#[derive(Debug)]
pub struct KeyCombo {
    // Must be held
    modifiers: HashMap<Key, KeyState>,
    // Must be pressed
    action_key: Key,
}

impl KeyCombo {
    pub fn new(action_key: Key, modifiers: &[Key]) -> Self {
        Self {
            modifiers: modifiers.iter().map(|key| (*key, KeyState::Up)).collect(),
            action_key,
        }
    }

    pub fn copy() -> Self {
        KeyCombo::new(Key::C, &[Key::Control])
    }

    pub fn key_down(&mut self, key: Key, mut on_trigger: impl FnMut()) -> Option<()> {
        if key == self.action_key && self.modifiers_pressed() {
            on_trigger()
        }

        let entry = self.modifiers.entry(key);
        if let Entry::Occupied(mut entry) = entry {
            let v = entry.get_mut();
            *v = KeyState::Down;
        }
        None
    }

    pub fn key_up(&mut self, key: Key) {
        let entry = self.modifiers.entry(key);
        if let Entry::Occupied(mut entry) = entry {
            let v = entry.get_mut();
            *v = KeyState::Up;
        }
    }

    fn modifiers_pressed(&self) -> bool {
        self.modifiers
            .iter()
            .all(|(_, state)| *state == KeyState::Down)
    }
}
