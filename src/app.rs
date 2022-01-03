use crate as lib;
use lib::DrawContext;
use lib::State;

pub trait App {
    fn init() -> Self;

    fn update(&mut self, state: &State, draw_context: &mut DrawContext);

    fn draw(&self, draw_context: &mut DrawContext);
}
