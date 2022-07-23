use crate::runtime::draw_context::DrawData;
use crate::runtime::state::InternalState;
use crate::{Button, Color, Resources};

#[derive(Debug)]
pub(crate) struct Pico8Impl {
    pub(crate) draw_data: DrawData,
    pub(crate) state: InternalState,
    pub(crate) resources: Resources,
}

impl Pico8Impl {
    pub(crate) fn new(draw_data: DrawData, state: InternalState, resources: Resources) -> Self {
        Self {
            draw_data,
            state,
            resources,
        }
    }
}

impl Pico8 for Pico8Impl {
    fn mget(&self, x: i32, y: i32) -> u8 {
        self.resources.map.mget(x, y)
    }

    fn mset(&mut self, x: i32, y: i32, spr: u8) {
        self.resources
            .map
            .mset(x.try_into().unwrap(), y.try_into().unwrap(), spr);
    }

    // TODO: Check we do the same left-to-right (or vice versa)
    // order as pico8
    // pub fn fget_n(&self, sprite: usize, flag: u8) -> bool {
    //     self.sprite_flags.fget_n(sprite, flag)
    // }

    fn fset(&mut self, sprite: usize, flag: usize, value: bool) -> u8 {
        self.resources.sprite_flags.fset(sprite, flag, value)
    }

    fn btnp(&self, button: Button) -> bool {
        self.state.button(button).btnp()
    }

    fn btn(&self, button: Button) -> bool {
        self.state.button(button).btn()
    }

    fn pal(&mut self, old: Color, new: Color) {
        todo!()
    }

    fn palt(&mut self, transparent_color: Option<Color>) {
        todo!()
    }

    fn reset_pal(&mut self) {
        todo!()
    }

    fn pset(&mut self, x: i32, y: i32, color: Color) {
        todo!()
    }

    fn cls(&mut self, color: Color) {
        todo!()
    }
    fn camera(&mut self, x: i32, y: i32) {
        todo!()
    }
    fn clip(&mut self, x: i32, y: i32, w: i32, h: i32) {
        todo!()
    }

    // todo
    fn map(&mut self, cell_x: i32, cell_y: i32, sx: i32, sy: i32, celw: i32, celh: i32, layer: u8) {
        todo!()
    }

    fn spr(&mut self, spr: usize, x: i32, y: i32) {
        todo!()
    }

    fn spr_(&mut self, spr: usize, x: i32, y: i32, w: f32, h: f32, flip_x: bool, flip_y: bool) {
        todo!()
    }

    fn fillp(&mut self) {
        todo!()
    }

    fn circ(&mut self, x: i32, y: i32, r: i32, color: Color) {
        todo!()
    }
    fn circfill(&mut self, x: i32, y: i32, r: i32, color: Color) {
        todo!()
    }

    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color) {
        todo!()
    }
    fn rectfill(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color) {
        todo!()
    }

    fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        todo!()
    }

    fn print(&mut self, text: &str, x: i32, y: i32, color: Color) {
        todo!()
    }

    // audio
    fn sfx(&mut self, sound_id: u8) {
        todo!()
    }
    fn music(&mut self, music_id: u8) {
        todo!()
    }

    fn append_camera(&mut self, x: i32, y: i32) {
        todo!()
    }

    fn mouse(&self) -> (i32, i32) {
        self.state.mouse()
    }
}

pub trait Pico8 {
    /// remap old color to new:
    ///   pico8.pal(7, 8)
    /// will cause white pixels to be rendered as red.
    // todo: add note that this doesn't apply to pset?
    fn pal(&mut self, old: Color, new: Color);
    fn palt(&mut self, transparent_color: Option<Color>);

    // TODO: Find better name?
    fn reset_pal(&mut self);

    fn pset(&mut self, x: i32, y: i32, color: Color);

    fn cls(&mut self, color: Color);
    fn camera(&mut self, x: i32, y: i32);
    fn clip(&mut self, x: i32, y: i32, w: i32, h: i32);

    fn mget(&self, x: i32, y: i32) -> u8;
    fn mset(&mut self, x: i32, y: i32, spr: u8);
    fn fset(&mut self, sprite: usize, flag: usize, value: bool) -> u8;

    fn map(&mut self, cell_x: i32, cell_y: i32, sx: i32, sy: i32, celw: i32, celh: i32, layer: u8);

    fn spr(&mut self, spr: usize, x: i32, y: i32);
    // TODO: Rename?
    fn spr_(&mut self, spr: usize, x: i32, y: i32, w: f32, h: f32, flip_x: bool, flip_y: bool);

    fn fillp(&mut self);

    fn circ(&mut self, x: i32, y: i32, r: i32, color: Color);
    fn circfill(&mut self, x: i32, y: i32, r: i32, color: Color);

    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color);
    fn rectfill(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color);

    fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color);

    fn print(&mut self, text: &str, x: i32, y: i32, color: Color);

    // audio
    fn sfx(&mut self, sound_id: u8);
    fn music(&mut self, music_id: u8);

    // input
    fn btnp(&self, button: Button) -> bool;
    fn btn(&self, button: Button) -> bool;

    fn append_camera(&mut self, x: i32, y: i32);

    fn mouse(&self) -> (i32, i32);
}

// Top level functions that pico8 provides that don't modify the global state.
// cos, sin, etc.
