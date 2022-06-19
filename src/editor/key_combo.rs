use std::collections::{hash_map::Entry, HashMap};

use crate::{Key, KeyState};

#[derive(Debug)]
pub struct KeyCombo<Id> {
    id: Id,
    // Must be held
    modifiers: HashMap<Key, KeyState>,
    // Must be pressed
    action_key: Key,
}

impl<Id: Copy> KeyCombo<Id> {
    pub fn new(id: Id, action_key: Key, modifiers: &[Key]) -> Self {
        Self {
            id,
            modifiers: modifiers.iter().map(|key| (*key, KeyState::Up)).collect(),
            action_key,
        }
    }

    pub fn copy(id: Id) -> Self {
        KeyCombo::new(id, Key::C, &[Key::Control])
    }

    pub fn paste(id: Id) -> Self {
        KeyCombo::new(id, Key::V, &[Key::Control])
    }

    pub fn key_down(&mut self, key: Key) -> Option<Id> {
        if key == self.action_key && self.modifiers_pressed() {
            return Some(self.id);
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
