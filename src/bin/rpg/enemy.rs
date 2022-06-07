use runty8::{
    runtime::{
        draw_context::{colors, DrawContext},
        sprite_sheet::Color,
    },
    ui::{DrawFn, Element},
};

use crate::Msg;

pub struct Enemy {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
    hp: i32,
    max_hp: i32,
    sprite: usize,
}

impl Enemy {
    pub fn new(x: i32, y: i32) -> Self {
        let max_hp = 13;

        Self {
            x,
            y,
            vx: 1,
            vy: 0,
            max_hp,
            hp: max_hp,
            sprite: 57,
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp = i32::max(self.hp - damage, 0);
    }

    pub fn view(&self) -> Element<'_, Msg> {
        DrawFn::new(move |draw| {
            draw.spr(self.sprite, self.x, self.y);
            self.view_hp_bar(draw)
        })
        .into()
    }

    fn view_hp_bar(&self, draw: &mut DrawContext) {
        let percentage_hp = self.hp as f32 / self.max_hp as f32;

        let base_width = 8;
        let filled_width = (percentage_hp * base_width as f32).round() as i32;

        let y = self.y + 9;

        Rect::new(self.x, y, base_width + 2, 4).fill(draw, colors::LIGHT_GREY);
        Rect::new(self.x, y, base_width + 2, 4).outline(draw, colors::WHITE);
        Rect::new(self.x + 1, y + 1, filled_width, 2).fill(draw, colors::RED);
    }

    pub fn update(&mut self) {
        if self.x > 121 {
            self.vx = -1;
        } else if self.x < 0 {
            self.vx = 1;
        }

        self.x += self.vx;
        self.y += self.vy;
    }
}

pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub fn outline(&self, draw_context: &mut DrawContext, color: Color) {
        if self.is_empty() {
            return;
        }
        draw_context.rect(
            self.x,
            self.y,
            self.x + self.w - 1,
            self.y + self.h - 1,
            color,
        )
    }

    pub fn fill(&self, draw_context: &mut DrawContext, color: Color) {
        if self.is_empty() {
            return;
        }

        draw_context.rectfill(
            self.x,
            self.y,
            self.x + self.w - 1,
            self.y + self.h - 1,
            color,
        )
    }

    fn is_empty(&self) -> bool {
        self.w <= 0 || self.h <= 0
    }
}
