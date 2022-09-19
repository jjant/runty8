use rand::Rng;
use std::f32::consts::PI;

use crate::runtime::draw_data::DrawData;
use crate::runtime::sprite_sheet::{Sprite, SpriteSheet};
use crate::runtime::state::State;
use crate::{Button, Color, Resources};

/// Struct providing an implementation of the pico8 API.
#[derive(Debug)]
pub struct Pico8 {
    pub(crate) draw_data: DrawData,
    pub(crate) state: State,
    pub(crate) resources: Resources,
    new_title: Option<String>,
}

impl Pico8 {
    pub(crate) fn new(draw_data: DrawData, state: State, resources: Resources) -> Self {
        Self {
            draw_data,
            state,
            resources,
            new_title: None,
        }
    }
}

// Public (Pico8) interface
impl Pico8 {
    pub fn mget(&self, x: i32, y: i32) -> u8 {
        self.resources.map.mget(x, y)
    }

    pub fn mset(&mut self, x: i32, y: i32, spr: u8) {
        self.resources
            .map
            .mset(x.try_into().unwrap(), y.try_into().unwrap(), spr);
    }

    // TODO: Check we do the same left-to-right (or vice versa)
    // order as pico8
    pub fn fget_n(&self, sprite: usize, flag: u8) -> bool {
        self.resources.sprite_flags.fget_n(sprite, flag)
    }

    pub fn fset(&mut self, sprite: usize, flag: usize, value: bool) -> u8 {
        self.resources.sprite_flags.fset(sprite, flag, value)
    }

    // TODO: Find a better naming scheme
    // TODO: Do we need to return the new flags?
    pub fn fset_all(&mut self, sprite: usize, flags: u8) {
        self.resources.sprite_flags.fset_all(sprite, flags);
    }

    pub fn btnp(&self, button: Button) -> bool {
        self.state.button(button).btnp()
    }

    pub fn btn(&self, button: Button) -> bool {
        self.state.button(button).btn()
    }

    pub fn pal(&mut self, old: Color, new: Color) {
        self.draw_data.pal(old, new);
    }

    pub fn palt(&mut self, transparent_color: Option<Color>) {
        self.draw_data.palt(transparent_color);
    }

    pub fn reset_pal(&mut self) {
        self.draw_data.reset_pal();
    }

    pub fn pset(&mut self, x: i32, y: i32, color: Color) {
        self.draw_data.pset(x, y, color);
    }

    pub fn cls(&mut self, color: Color) {
        self.draw_data.cls_color(color);
    }

    pub fn camera(&mut self, x: i32, y: i32) {
        self.draw_data.camera(x, y);
    }

    pub fn clip(&mut self, _x: i32, _y: i32, _w: i32, _h: i32) {
        todo!()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn map(
        &mut self,
        cell_x: i32,
        cell_y: i32,
        sx: i32,
        sy: i32,
        celw: i32,
        celh: i32,
        layer: u8,
    ) {
        self.draw_data.map(
            cell_x,
            cell_y,
            sx,
            sy,
            celw,
            celh,
            layer,
            &self.resources.map,
            &self.resources.sprite_flags,
            &self.resources.sprite_sheet,
        );
    }

    pub fn spr(&mut self, spr: usize, x: i32, y: i32) {
        self.spr_(spr, x, y, 1.0, 1.0, false, false);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn spr_(&mut self, spr: usize, x: i32, y: i32, w: f32, h: f32, flip_x: bool, flip_y: bool) {
        spr_from(
            &mut self.draw_data,
            &self.resources.sprite_sheet,
            spr,
            x,
            y,
            w,
            h,
            flip_x,
            flip_y,
        );
    }

    // TODO: Test
    pub fn sset(&mut self, x: i32, y: i32, color: Color) {
        if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
            self.resources.sprite_sheet.set(x, y, color);
        }
    }

    pub fn fillp(&mut self) {
        todo!()
    }

    pub fn circ(&mut self, x: i32, y: i32, r: i32, color: Color) {
        self.draw_data.circ(x, y, r, color);
    }
    pub fn circfill(&mut self, x: i32, y: i32, r: i32, color: Color) {
        self.draw_data.circfill(x, y, r, color);
    }

    pub fn rect(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        self.draw_data.rect(x0, y0, x1, y1, color);
    }

    pub fn rectfill(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        self.draw_data.rectfill(x0, y0, x1, y1, color);
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        self.draw_data.line(x0, y0, x1, y1, color);
    }

    pub fn print(&mut self, text: &str, x: i32, y: i32, color: Color) {
        self.draw_data.print(text, x, y, color);
    }

    // audio
    pub fn sfx(&mut self, _sound_id: u8) {
        todo!()
    }
    pub fn music(&mut self, _music_id: u8) {
        todo!()
    }

    // Non-standard stuf
    pub fn append_camera(&mut self, x: i32, y: i32) {
        self.draw_data.append_camera(x, y);
    }

    pub fn mouse(&self) -> (i32, i32) {
        self.state.mouse()
    }

    pub fn set_title(&mut self, new_title: String) {
        self.new_title = Some(new_title);
    }
}

fn spr_from(
    draw_data: &mut DrawData,
    sprite_sheet: &SpriteSheet,
    spr: usize,
    x: i32,
    y: i32,
    w: f32,
    h: f32,
    flip_x: bool,
    flip_y: bool,
) {
    draw_data.spr_(spr, sprite_sheet, x, y, w, h, flip_x, flip_y);
}

// Utility pub(crate) methods
impl Pico8 {
    // TODO: Remove this `allow` when we use it in the editor.
    #[allow(dead_code)]
    pub(crate) fn spr_from(
        &mut self,
        sprite_sheet: &SpriteSheet,
        spr: usize,
        x: i32,
        y: i32,
        w: f32,
        h: f32,
        flip_x: bool,
        flip_y: bool,
    ) {
        spr_from(
            &mut self.draw_data,
            sprite_sheet,
            spr,
            x,
            y,
            w,
            h,
            flip_x,
            flip_y,
        );
    }

    pub(crate) fn raw_spr(&mut self, sprite: &Sprite, x: i32, y: i32) {
        self.draw_data.raw_spr(sprite, x, y);
    }

    pub(crate) fn take_new_title(&mut self) -> Option<String> {
        self.new_title.take()
    }
}

// Top level functions that pico8 provides that don't modify the global state.
// cos, sin, etc.

/// <https://pico-8.fandom.com/wiki/Sin>
pub fn sin(f: f32) -> f32 {
    (-f * 2.0 * PI).sin()
}

/// <https://pico-8.fandom.com/wiki/Rnd>
pub fn rnd(limit: f32) -> f32 {
    rand::thread_rng().gen_range(0.0..limit)
}

/// <https://pico-8.fandom.com/wiki/Mid>
pub fn mid(min: f32, val: f32, max: f32) -> f32 {
    let mut arr = [min, val, max];
    arr.sort_by(|a, b| a.partial_cmp(b).unwrap());

    arr[1]
}

#[cfg(test)]
mod tests {
    use super::{mid, rnd, sin};

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d && $y - $x < $d) {
                panic!();
            }
        };
    }

    #[test]
    fn sin_works() {
        assert_delta!(sin(0.0), 0.0, 0.00001);
        assert_delta!(sin(0.125), -0.70710677, 0.00001);
        assert_delta!(sin(0.25), -1.0, 0.00001);
        assert_delta!(sin(0.375), -0.70710677, 0.00001);
        assert_delta!(sin(0.5), 0.0, 0.00001);
        assert_delta!(sin(0.625), 0.70710677, 0.00001);
        assert_delta!(sin(0.75), 1.0, 0.00001);
        assert_delta!(sin(0.875), 0.70710677, 0.00001);
        assert_delta!(sin(1.0), 0.0, 0.00001);
    }

    #[test]
    fn rnd_works() {
        for _ in 0..100 {
            let random_value = rnd(50.0);

            assert!(0.0 < random_value && random_value < 50.0);
        }
    }

    #[test]
    fn mid_works() {
        assert_delta!(mid(8.0, 2.0, 4.0), 4.0, 0.00001);
        assert_delta!(mid(-3.5, -3.4, -3.6), -3.5, 0.00001);
        assert_delta!(mid(6.0, 6.0, 8.0), 6.0, 0.00001);
        assert_delta!(mid(0.0, 2.0, 1.0), 1.0, 0.00001);
    }
}
