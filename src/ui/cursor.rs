use super::Widget;
use crate::{editor, DrawContext, Sprite};
use std::marker::PhantomData;

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

impl<'a, Msg: Copy> Cursor<'a, Msg> {
    pub fn new(state: &'a mut State) -> Self {
        Self {
            state,
            pd: PhantomData,
        }
    }
}

impl<'a, Msg: Copy> Widget for Cursor<'a, Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        _: crate::Event,
        cursor_position: (i32, i32),
        _: &mut impl FnMut(Self::Msg),
    ) {
        self.state.cursor_position = cursor_position;
    }

    fn draw(&self, draw: &mut DrawContext) {
        draw.raw_spr(
            Sprite::new(editor::MOUSE_SPRITE),
            self.state.cursor_position.0 - 3,
            self.state.cursor_position.1 - 2,
        );
    }
}
