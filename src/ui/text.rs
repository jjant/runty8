use std::{fmt::Debug, marker::PhantomData};

use crate::Color;

use super::{DispatchEvent, Widget};

pub struct Text<Msg> {
    x: i32,
    y: i32,
    text: String,
    color: Color,
    pd: PhantomData<Msg>,
}

impl<Msg> Text<Msg> {
    pub fn new(text: String, x: i32, y: i32, color: Color) -> Box<Self> {
        Box::new(Self {
            x,
            y,
            text,
            color,
            pd: PhantomData,
        })
    }
}

impl<'a, Msg: Copy + Debug> Widget for Text<Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        _event: crate::Event,
        _cursor_position: (i32, i32),
        _dispatch_event: &mut DispatchEvent<Self::Msg>,
    ) {
    }

    fn draw(&self, draw: &mut crate::DrawContext) {
        draw.print(&self.text, self.x, self.y, self.color);
    }
}
