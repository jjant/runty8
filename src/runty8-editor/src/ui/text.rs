use crate::Pico8;
use std::{fmt::Debug, marker::PhantomData};

use crate::Color;

use super::{DispatchEvent, Widget};

pub struct Text<'a, Msg> {
    text: &'a str,
    x: i32,
    y: i32,
    color: Color,
    pd: PhantomData<Msg>,
}

impl<'a, Msg> Text<'a, Msg> {
    pub fn new(text: &'a str, x: i32, y: i32, color: Color) -> Self {
        Self {
            x,
            y,
            text,
            color,
            pd: PhantomData,
        }
    }
}

impl<'a, Msg: Copy + Debug> Widget for Text<'a, Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        _event: runty8_core::Event,
        _cursor_position: (i32, i32),
        _dispatch_event: &mut DispatchEvent<Self::Msg>,
    ) {
    }

    fn draw(&mut self, draw: &mut Pico8) {
        draw.print(self.text, self.x, self.y, self.color);
    }
}
