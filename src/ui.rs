pub mod cursor;

use crate::{DrawContext, Event, MouseButton, MouseEvent};
use enum_dispatch::enum_dispatch;

use self::cursor::Cursor;

pub struct Button<'a, Msg> {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    on_press: Option<Msg>,
    state: &'a mut ButtonState,
}

#[derive(Debug)]
pub struct ButtonState {
    pressed: bool,
}

impl ButtonState {
    pub fn new() -> Self {
        Self { pressed: false }
    }
}

impl<'a, Msg> Button<'a, Msg> {
    pub fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        on_press: Option<Msg>,
        state: &'a mut ButtonState,
    ) -> Self {
        Button {
            x,
            y,
            width,
            height,
            on_press,
            state,
        }
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        let contains_x = x >= self.x && x < self.width;
        let contains_y = y >= self.y && y < self.height;

        contains_x && contains_y
    }
}

#[enum_dispatch]
pub trait Widget {
    type Msg: Copy;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut impl FnMut(Self::Msg),
    );

    fn draw(&self, draw: &mut DrawContext);
}

impl<'a, Msg: Copy> Widget for Button<'a, Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut impl FnMut(Self::Msg),
    ) {
        use Event::*;
        use MouseEvent::*;

        match event {
            Mouse(Down(MouseButton::Left)) => {
                if self.contains(cursor_position.0, cursor_position.1) {
                    self.state.pressed = true;
                }
            }
            Mouse(Up(MouseButton::Left)) => {
                if self.contains(cursor_position.0, cursor_position.1) && self.state.pressed {
                    if let Some(on_press) = self.on_press {
                        dispatch_event(on_press);
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
    }
}

impl<T: Widget> Widget for Vec<T> {
    type Msg = T::Msg;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut impl FnMut(Self::Msg),
    ) {
        for w in self.iter_mut() {
            w.on_event(event, cursor_position, dispatch_event);
        }
    }

    fn draw(&self, draw: &mut DrawContext) {
        for w in self.iter() {
            w.draw(draw);
        }
    }
}

pub enum WidgetImpl<'a, Msg> {
    Tree(Vec<WidgetImpl<'a, Msg>>),
    Cursor(Cursor<'a, Msg>),
    Button(Button<'a, Msg>),
}

impl<'a, Msg: Copy> Widget for WidgetImpl<'a, Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut impl FnMut(Self::Msg),
    ) {
        match self {
            WidgetImpl::Tree(x) => x.on_event(event, cursor_position, dispatch_event),
            WidgetImpl::Button(x) => x.on_event(event, cursor_position, dispatch_event),
            WidgetImpl::Cursor(x) => x.on_event(event, cursor_position, dispatch_event),
        }
    }

    fn draw(&self, draw: &mut DrawContext) {
        match self {
            WidgetImpl::Tree(x) => x.draw(draw),
            WidgetImpl::Button(x) => x.draw(draw),
            WidgetImpl::Cursor(x) => x.draw(draw),
        }
    }
}

pub trait ElmApp2 {
    type Msg: Copy;

    fn init() -> Self;

    fn update(&mut self, msg: &Self::Msg);

    fn view(&mut self) -> WidgetImpl<Self::Msg>;
}

pub fn run_app2<T: ElmApp2 + 'static>() {
    let state = crate::State::new();
    let draw_context = DrawContext::new(state);

    crate::screen::do_something::<T>(draw_context);
}
