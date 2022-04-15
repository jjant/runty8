use crate as lib;
use lib::DrawContext;
use lib::State;

pub trait App {
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
