use super::{DispatchEvent, Widget};
use crate::{editor, runtime::draw_context::DrawContext, Sprite};
use std::{fmt::Debug, marker::PhantomData};

pub struct Cursor<'a, Msg> {
    state: &'a mut State,
    pd: PhantomData<Msg>,
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
        _: crate::Event,
        cursor_position: (i32, i32),
        _: &mut DispatchEvent<Self::Msg>,
    ) {
        self.state.cursor_position = cursor_position;
    }

    fn draw(&self, draw: &mut DrawContext) {
        draw.palt(Some(0));
        draw.raw_spr(
            Sprite::new(editor::MOUSE_SPRITE),
            self.state.cursor_position.0 - 3,
            self.state.cursor_position.1 - 2,
        );
    }
}
