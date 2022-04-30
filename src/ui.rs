pub mod button;
pub mod cursor;
pub mod text;

use std::{fmt::Debug, marker::PhantomData};

use crate::{runtime::cmd::Cmd, DrawContext, Event};
use enum_dispatch::enum_dispatch;

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

pub struct Tree<'a, Msg> {
    elements: Vec<Box<dyn Widget<Msg = Msg> + 'a>>,
}

impl<'a, Msg> Tree<'a, Msg> {
    pub fn new(elements: Vec<Box<dyn Widget<Msg = Msg> + 'a>>) -> Self {
        Self { elements }
    }
}

impl<'a, Msg: Copy + Debug> Widget for Tree<'a, Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut DispatchEvent<Self::Msg>,
    ) {
        for w in self.elements.iter_mut() {
            w.on_event(event, cursor_position, dispatch_event);
        }
    }

    fn draw(&self, draw: &mut DrawContext) {
        for w in self.elements.iter() {
            w.draw(draw);
        }
    }
}

pub struct DrawFn<Msg> {
    pd: PhantomData<Msg>,
    f: Box<dyn Fn(&mut DrawContext)>,
}

impl<Msg> DrawFn<Msg> {
    pub fn new(f: impl Fn(&mut DrawContext) + 'static) -> Box<Self> {
        Box::new(Self {
            f: Box::new(f),
            pd: PhantomData,
        })
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

    fn update(&mut self, msg: &Self::Msg) -> Cmd<Self::Msg>;

    fn view(&mut self) -> Tree<'_, Self::Msg>;

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
