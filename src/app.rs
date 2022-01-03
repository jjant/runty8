use crate as lib;
use lib::DrawContext;
use lib::State;

pub trait App {
    fn init() -> Self;

    fn update(&mut self, state: &State);

    fn draw(&self, draw_context: &mut DrawContext);
}

pub(crate) trait DevApp {
    fn init() -> Self;

    fn update(&mut self, state: &mut State);

    fn draw(&self, draw_context: &mut DrawContext);
}
