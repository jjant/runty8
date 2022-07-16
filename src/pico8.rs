use crate::{Button, Color};

pub trait Pico8 {
    /// Remap old color to new:
    ///   pico8.pal(7, 8)
    /// Will cause white pixels to be rendered as red.
    // TODO: Add note that this doesn't apply to pset?
    fn pal(&mut self, old: Color, new: Color);

    fn pset(&mut self, x: i32, y: i32, color: Color);

    fn cls(&mut self, color: Color);
    fn camera(&mut self, x: i32, y: i32);
    fn clip(&mut self, x: i32, y: i32, w: i32, h: i32);
    // todo
    fn map(&mut self);

    fn spr(&mut self, spr: u8, x: i32, y: i32);

    fn fillp(&mut self);

    fn circ(&mut self, x: i32, y: i32, r: i32, color: Color);
    fn circfill(&mut self, x: i32, y: i32, r: i32, color: Color);

    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color);
    fn rectfill(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color);

    fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color);

    fn print(&mut self, text: &str, x: i32, y: i32, color: Color);

    // Audio
    fn sfx(&mut self, sound_id: u8);
    fn music(&mut self, music_id: u8);

    // Input
    fn btnp(&mut self, button: Button) -> bool;
    fn btn(&mut self, button: Button) -> bool;
}

// Top level functions that pico8 provides that don't modify the global state.
// cos, sin, etc.
