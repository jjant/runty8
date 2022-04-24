use crate::{DrawContext, Event, MouseButton};

use super::{DispatchEvent, Widget};
use std::fmt::Debug;

pub struct Button<'a, Msg> {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    on_press: Option<Msg>,
    state: &'a mut State,
    content: Box<dyn Widget<Msg = Msg>>,
}

#[derive(Debug)]
pub struct State {
    pressed: bool,
}

impl State {
    pub fn new() -> Self {
        Self { pressed: false }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Msg> Button<'a, Msg> {
    pub fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        on_press: Option<Msg>,
        state: &'a mut State,
        content: Box<dyn Widget<Msg = Msg>>,
    ) -> Self {
        Button {
            x,
            y,
            width,
            height,
            on_press,
            state,
            content,
        }
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        let contains_x = x >= self.x && x < self.x + self.width;
        let contains_y = y >= self.y && y < self.y + self.height;

        contains_x && contains_y
    }
}

impl<'a, Msg: Copy + Debug> Widget for Button<'a, Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut DispatchEvent<'_, Msg>,
    ) {
        use crate::MouseEvent::*;
        use Event::*;

        // TODO: Dispatch events for content?
        match event {
            Mouse(Down(MouseButton::Left)) => {
                if self.contains(cursor_position.0, cursor_position.1) {
                    self.state.pressed = true;
                }
            }
            Mouse(Up(MouseButton::Left)) => {
                if self.contains(cursor_position.0, cursor_position.1) && self.state.pressed {
                    if let Some(on_press) = self.on_press {
                        dispatch_event.call(on_press);
                    }
                }

                self.state.pressed = false;
            }
            _ => {}
        }
    }

    fn draw(&self, draw: &mut DrawContext) {
        let color = if self.state.pressed { 5 } else { 9 };

        // TODO: Handle properly
        draw.rectfill(
            self.x,
            self.y,
            self.x + self.width - 1,
            self.y + self.height - 1,
            color,
        );

        // TODO: Offset contents so that they're based off of the button's position?
        //
        // i.e, in `button {x = 23, y = 42} [ content { x = 0, y = 0 } ]`
        // content should be drawn at the right top corner of the button? Probably?
        self.content.draw(draw);
    }
}
