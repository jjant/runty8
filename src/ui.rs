pub mod button;
pub mod cursor;
pub mod text;
use crate::{
    runtime::{
        cmd::Cmd,
        draw_context::{DrawContext, DrawData},
    },
    Event,
};
use std::{fmt::Debug, marker::PhantomData};

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

pub type Element<'a, Msg> = Box<dyn Widget<Msg = Msg> + 'a>;

impl<'a, Msg: Copy + Debug + 'a> From<Vec<Element<'a, Msg>>> for Element<'a, Msg> {
    fn from(val: Vec<Element<'a, Msg>>) -> Self {
        Tree::<'a, Msg>::new(val)
    }
}

impl<'a, Msg> Tree<'a, Msg> {
    pub fn new(elements: Vec<Box<dyn Widget<Msg = Msg> + 'a>>) -> Box<Self> {
        Box::new(Self { elements })
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
pub trait ElmApp2
where
    Self: Sized,
{
    type Msg: Copy + Debug;
    type Flags;

    fn init(flags: Self::Flags) -> (Self, Cmd<Self::Msg>);

    fn update(&mut self, msg: &Self::Msg) -> Cmd<Self::Msg>;

    fn view(&mut self) -> Element<'_, Self::Msg>;

    fn subscriptions(&self) -> Sub<Self::Msg>;
}

pub enum Sub<Msg> {
    OnAnimationFrame(fn(f32) -> Msg),
    NoSub,
}
