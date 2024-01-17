use super::{DispatchEvent, Widget};
use crate::pico8::Pico8EditorExt as _;
use runty8_core::Pico8;
use std::{fmt::Debug, marker::PhantomData};

pub struct Cursor<'a, Msg> {
    state: &'a mut State,
    pd: PhantomData<Msg>,
}

impl<Msg> Cursor<'_, Msg> {
    const POINTER_SPRITE: usize = 48;
    // TODO: Use target sprite.
    #[allow(dead_code)]
    const TARGET_SPRITE: usize = 49;
}

#[derive(Debug)]
pub struct State {
    cursor_position: (i32, i32),
}

impl State {
    pub fn new() -> Self {
        Self {
            cursor_position: (63, 63),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Msg: Copy> Cursor<'a, Msg> {
    pub fn new(state: &'a mut State) -> Self {
        Self {
            state,
            pd: PhantomData,
        }
    }
}

impl<'a, Msg: Copy + Debug> Widget for Cursor<'a, Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        _: runty8_core::Event,
        cursor_position: (i32, i32),
        _: &mut DispatchEvent<Self::Msg>,
    ) {
        self.state.cursor_position = cursor_position;
    }

    fn draw(&mut self, pico8: &mut Pico8) {
        pico8.palt(Some(0));
        pico8.editor_spr(
            Self::POINTER_SPRITE,
            self.state.cursor_position.0 - 3,
            self.state.cursor_position.1 - 1,
        );
    }
}
