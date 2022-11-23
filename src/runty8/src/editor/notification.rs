use crate::ui::DispatchEvent;
use crate::ui::Widget;
use crate::Pico8;
use runty8_runtime::{colors, Event};
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct State {
    timer: i32,
    enter_state: EnterState,
    content: String,
}

#[derive(Debug, PartialEq)]
enum EnterState {
    Entering,
    Displaying,
    Leaving,
    Left,
}

impl EnterState {
    fn duration(&self) -> i32 {
        match self {
            Entering => 8,
            Displaying => 90,
            Leaving => 8,
            Left => 0,
        }
    }

    fn next(&self) -> Option<EnterState> {
        match self {
            Entering => Some(Displaying),
            Displaying => Some(Leaving),
            Leaving => Some(Left),
            Left => None,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            timer: Left.duration(),
            enter_state: Left,
            content: "".to_owned(),
        }
    }

    pub fn alert(&mut self, content: String) {
        self.content = content;
        self.reset()
    }

    fn reset(&mut self) {
        self.set_state(Entering)
    }

    fn set_state(&mut self, state: EnterState) {
        self.timer = state.duration();
        self.enter_state = state;
    }

    fn tick(&mut self) {
        match self.enter_state {
            Left => {}
            _ => {
                self.timer -= 1;
                if self.timer <= 0 {
                    if let Some(next_state) = self.enter_state.next() {
                        self.set_state(next_state);
                    }
                }
            }
        }
    }

    #[cfg(test)]
    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct Notification<'a, Msg> {
    state: &'a mut State,
    phantom: PhantomData<Msg>,
}

use EnterState::*;

impl<'a, Msg> Notification<'a, Msg> {
    pub fn new(state: &'a mut State) -> Self {
        Self {
            state,
            phantom: PhantomData,
        }
    }
}

impl<'a, Msg: Copy + Debug> Widget for Notification<'a, Msg> {
    type Msg = Msg;

    fn on_event(&mut self, event: Event, _: (i32, i32), _: &mut DispatchEvent<Self::Msg>) {
        let state = &mut self.state;

        if let Event::Tick { .. } = event {
            state.tick()
        }
    }

    fn draw(&mut self, draw: &mut Pico8) {
        let x = 1;
        let base_y = 122;
        let offset_y_max = 8;
        let timer_ratio = self.state.timer as f32 / self.state.enter_state.duration() as f32;

        let offset_y = (offset_y_max as f32
            * match self.state.enter_state {
                Entering => timer_ratio,
                Displaying => 0.0,
                Leaving => 1.0 - timer_ratio,
                Left => return,
            }) as i32;

        let y = base_y + offset_y;

        // Cover regular bar messages
        draw.rectfill(0, 121, 127, 127, colors::RED);
        draw.print(&self.state.content, x, y, colors::LIGHT_PEACH);
    }
}
