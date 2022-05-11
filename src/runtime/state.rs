use std::cell::Cell;

use super::{map::Map, sprite_sheet::SpriteSheet};
use crate::screen::Keys;
use itertools::Itertools;
use ButtonState::*;

#[derive(Debug)]
pub struct Flags {
    flags: [Cell<u8>; SpriteSheet::SPRITE_COUNT],
}

impl Flags {
    pub fn file_name() -> &'static str {
        "sprite_flags.txt"
    }

    pub fn new() -> Self {
        let flags = vec![Cell::new(0); SpriteSheet::SPRITE_COUNT]
            .try_into()
            .unwrap();

        Self { flags }
    }

    pub(crate) fn with_flags(flags: [u8; SpriteSheet::SPRITE_COUNT]) -> Self {
        Self {
            flags: flags.map(Cell::new),
        }
    }

    fn len(&self) -> usize {
        self.flags.len()
    }

    fn set(&self, index: usize, value: u8) {
        self.flags[index].set(value);
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        // TODO: Check what pico8 does in cases when the index is out of bounds
        let cell = self.flags.get(index)?;

        Some(cell.get())
    }

    pub fn fset(&self, sprite: usize, flag: usize, value: bool) -> u8 {
        // TODO: Check what pico8 does in these cases:
        assert!(flag <= 7);

        let value = value as u8;
        let mut flags = self.get(sprite).unwrap();
        flags = (flags & !(1u8 << flag)) | (value << flag);

        self.set(sprite, flags);

        flags
    }

    pub fn fget_n(&self, sprite: usize, flag: u8) -> bool {
        // TODO: Check what pico8 does in these cases:
        assert!(sprite < self.len());
        assert!(flag <= 7);

        let res = (self.get(sprite).unwrap() & (1 << flag)) >> flag;
        assert!(res == 0 || res == 1);

        res != 0
    }

    pub fn deserialize(file_contents: &str) -> Result<Self, String> {
        let flags_vec: Result<Vec<u8>, String> = file_contents
            .lines()
            .map(|line| line.parse::<u8>().map_err(|e| format!("{:?}", e)))
            .collect();

        let flags_array: [u8; SpriteSheet::SPRITE_COUNT] =
            flags_vec?.try_into().map_err(|v: Vec<u8>| {
                format!(
                    "Incorrect number of elements, needed: {}, got: {}",
                    SpriteSheet::SPRITE_COUNT,
                    v.len()
                )
            })?;

        Ok(Self::with_flags(flags_array))
    }

    pub(crate) fn serialize(&self) -> String {
        self.flags
            .iter()
            .map(Cell::get)
            .map(|flag| flag.to_string())
            .join("\n")
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct State<'map, 'flags> {
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
    pub(crate) sprite_flags: &'flags Flags,
    pub(crate) map: &'map Map,
    pub(crate) assets_path: &'static str,
}

impl<'map, 'flags> State<'map, 'flags> {
    // TODO: Make pub(crate)
    pub fn new(
        assets_path: &'static str,
        sprite_sheet: SpriteSheet,
        sprite_flags: &'flags Flags,
        map: &'map Map,
    ) -> Self {
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
            sprite_flags,
            map,
            assets_path,
        }
    }

    pub fn mget(&self, cel_x: usize, cel_y: usize) -> u8 {
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

// TODO: Implement properly
// TODO2: I think this is fine, now?
#[derive(Debug)]
pub(crate) enum ButtonState {
    JustPressed,
    Held,
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

#[derive(Debug)]
pub enum Scene {
    Editor,
    App,
}

impl Scene {
    fn initial() -> Self {
        Scene::App
    }

    pub fn flip(&mut self) {
        *self = match self {
            Scene::Editor => Scene::App,
            Scene::App => Scene::Editor,
        }
    }
}
