use crate as lib;
use lib::DrawContext;
use lib::State;

pub trait App {
    fn init() -> Self;

    fn draw(&self, draw_context: &mut DrawContext);

    fn update(&mut self, state: &State);
}
