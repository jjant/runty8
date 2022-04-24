pub mod button;
pub mod cursor;
pub mod text;

use std::{fmt::Debug, marker::PhantomData};

use crate::{App, DrawContext, Event};
use enum_dispatch::enum_dispatch;

use self::{button::Button, cursor::Cursor, text::Text};

pub struct DispatchEvent<'a, Msg> {
    queue: &'a mut Vec<Msg>,
}

impl<'a, Msg> DispatchEvent<'a, Msg> {
    pub fn new(queue: &'a mut Vec<Msg>) -> Self {
        Self { queue }
    }

    pub fn call(&mut self, msg: Msg) {
        self.queue.push(msg);
    }
}

#[enum_dispatch]
pub trait Widget {
    type Msg: Copy + Debug;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut DispatchEvent<Self::Msg>,
    );

    fn draw(&self, draw: &mut DrawContext);
}

impl<T: Widget> Widget for Vec<T> {
    type Msg = T::Msg;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut DispatchEvent<Self::Msg>,
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
    Text(Text<Msg>),
    DrawFn(DrawFn<Msg>),
}

impl<'a, Msg: Copy + Debug> Widget for WidgetImpl<'a, Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut DispatchEvent<Self::Msg>,
    ) {
        match self {
            WidgetImpl::Tree(x) => x.on_event(event, cursor_position, dispatch_event),
            WidgetImpl::Button(x) => x.on_event(event, cursor_position, dispatch_event),
            WidgetImpl::Cursor(x) => x.on_event(event, cursor_position, dispatch_event),
            WidgetImpl::Text(x) => x.on_event(event, cursor_position, dispatch_event),
            WidgetImpl::DrawFn(x) => x.on_event(event, cursor_position, dispatch_event),
        }
    }

    fn draw(&self, draw: &mut DrawContext) {
        match self {
            WidgetImpl::Tree(x) => x.draw(draw),
            WidgetImpl::Button(x) => x.draw(draw),
            WidgetImpl::Cursor(x) => x.draw(draw),
            WidgetImpl::Text(x) => x.draw(draw),
            WidgetImpl::DrawFn(x) => x.draw(draw),
        }
    }
}

pub struct DrawFn<Msg> {
    f: fn(draw: &mut DrawContext),
    pd: PhantomData<Msg>,
}

impl<Msg> DrawFn<Msg> {
    pub fn new(f: fn(draw: &mut DrawContext)) -> Self {
        Self { f, pd: PhantomData }
    }
}

impl<Msg: Copy + Debug> Widget for DrawFn<Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        _event: Event,
        _cursor_position: (i32, i32),
        _dispatch_event: &mut DispatchEvent<Self::Msg>,
    ) {
    }

    fn draw(&self, draw: &mut DrawContext) {
        (self.f)(draw);
    }
}
pub trait ElmApp2 {
    type Msg: Copy + Debug;

    fn init() -> Self;

    fn update(&mut self, msg: &Self::Msg);

    fn view(&mut self) -> WidgetImpl<Self::Msg>;

    fn subscriptions(&self) -> Sub<Self::Msg>;
}

pub enum Sub<Msg> {
    OnAnimationFrame(fn(f32) -> Msg),
    NoSub,
}

pub fn run_app2<T: ElmApp2 + 'static>() {
    let state = crate::State::new();
    let draw_context = DrawContext::new(state);

    crate::screen::do_something::<T>(draw_context);
}
