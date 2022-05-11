use crate::{self as lib, runtime::draw_context::DrawContext};
// pub mod pico8;
use lib::State;

pub trait App {
    fn init() -> Self;
    fn update(&mut self, state: &State);
    fn draw(&self, draw_context: &mut DrawContext);
}

impl<T: App> ElmApp for T {
    type Action = ();

    fn init() -> Self {
        <Self as App>::init()
    }

    fn update(&mut self, state: &State, _: &[Self::Action]) {
        <Self as App>::update(self, state);
    }

    fn draw(&mut self, draw_context: &mut DrawContext) -> Vec<Self::Action> {
        <Self as App>::draw(self, draw_context);
        vec![]
    }
}

pub trait ElmApp {
    type Action;

    fn init() -> Self;

    fn update(&mut self, state: &State, actions: &[Self::Action]);

    fn draw(&mut self, draw_context: &mut DrawContext) -> Vec<Self::Action>;
}

pub(crate) trait DevApp {
    fn init() -> Self;

    fn update(&mut self, state: &mut State);

    fn draw(&self, draw_context: &mut DrawContext);
}
