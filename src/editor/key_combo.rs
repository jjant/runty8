use std::collections::{hash_map::Entry, HashMap};

use crate::{Key, KeyState, KeyboardEvent};

#[derive(Debug)]
pub struct KeyCombos<Id> {
    key_combos: Vec<KeyCombo<Id>>,
}

impl<Id> KeyCombos<Id> {
    pub fn new() -> Self {
        Self { key_combos: vec![] }
    }

    pub fn push(mut self, key_combo: KeyCombo<Id>) -> Self {
        self.key_combos.push(key_combo);

        self
    }
}

impl<Id: Copy> KeyCombos<Id> {
    pub fn on_event(&mut self, key_event: KeyboardEvent, mut on_combo: impl FnMut(Id)) {
        let mut handled = false;
        for key_combo in self.key_combos.iter_mut() {
            match key_event.state {
                KeyState::Up => key_combo.key_up(key_event.key),
                KeyState::Down => {
                    if let Some(action_id) = key_combo.key_down(key_event.key) {
                        if !handled {
                            handled = true;
                            on_combo(action_id)
                        }
                    }
                }
            }
        }
    }
}

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
