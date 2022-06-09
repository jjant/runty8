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
    speed_x: i32,
    vy: i32,
    hp: i32,
    max_hp: i32,
    sprite: usize,
    flash_timer: i32,
    damage: i32,
    damage_counter: f32,
}

impl Enemy {
    pub fn new(x: i32, y: i32, sprite: usize) -> Self {
        let max_hp = 10;
        let speed_x = 1;

        Self {
            x,
            y,
            vx: speed_x,
            speed_x,
            vy: 0,
            max_hp,
            hp: max_hp,
            sprite,
            flash_timer: 0,
            damage: 0,
            damage_counter: 0.0,
        }
    }

    pub fn mage(x: i32, y: i32) -> Self {
        Self::new(x, y, 57)
    }

    pub fn snail(x: i32, y: i32) -> Self {
        Self::new(x, y, 59)
    }

    pub fn take_damage(&mut self, damage: i32) {
        let new_hp = i32::max(self.hp - damage, 0);
        let actual_damage = self.hp - new_hp;

        self.hp = new_hp;

        self.flash_timer += actual_damage * 15;
        self.damage += actual_damage;
        self.damage_counter += actual_damage as f32;
    }

    pub fn view(&self) -> Element<'_, Msg> {
        DrawFn::new(move |draw| {
            self.view_sprite(draw);
            self.view_hp_bar(draw);
        })
        .into()
    }

    fn view_sprite(&self, draw: &mut DrawContext) {
        if self.flash_timer > 0 && self.flash_timer % 2 == 0 {
            draw.pal(3, 7);
            draw.pal(1, 10);
            draw.pal(6, 9);
        }
        draw.spr(self.sprite, self.x, self.y);
        draw.reset_pal();
    }

    fn view_hp_bar(&self, draw: &mut DrawContext) {
        const BASE_WIDTH: i32 = 30;
        const BASE_HEIGHT: i32 = 4;
        const BORDER_WIDTH: i32 = 1;

        let percentage_hp = self.hp as f32 / self.max_hp as f32;

        let filled_width = (percentage_hp * BASE_WIDTH as f32).round() as i32;

        let y = self.y - 6;

        let containing_rect = Rect::centered(
            self.x + 4,
            y,
            BASE_WIDTH + 2 * BORDER_WIDTH,
            BASE_HEIGHT + 2 * BORDER_WIDTH,
        );
        containing_rect.fill(draw, colors::LIGHT_GREY);
        containing_rect.outline(draw, colors::WHITE);
        let current_hp_rect = Rect::new(
            containing_rect.x + 1,
            containing_rect.y + 1,
            filled_width,
            BASE_HEIGHT,
        );
        current_hp_rect.fill(draw, colors::RED);

        let percentage_damage = self.damage_counter / self.max_hp as f32;
        let damage_width = (percentage_damage * BASE_WIDTH as f32).round() as i32;
        Rect::new(
            current_hp_rect.right() + 1,
            containing_rect.y + 1,
            damage_width,
            BASE_HEIGHT,
        )
        .fill(draw, colors::ORANGE);
    }

    pub fn update(&mut self) {
        if self.x > 121 {
            self.vx = -self.speed_x;
        } else if self.x < 0 {
            self.vx = self.speed_x;
        }

        self.x += self.vx;
        self.y += self.vy;

        self.handle_damage_animation();
    }

    fn handle_damage_animation(&mut self) {
        let counter_duration = 20.0;

        self.flash_timer = i32::max(self.flash_timer - 1, 0);
        self.damage_counter = f32::max(
            self.damage_counter - self.damage as f32 / counter_duration,
            0.0,
        );

        if self.damage_counter == 0.0 {
            self.damage = 0;
        }
    }
}

pub struct Rect {
    // x, y: position of the top left corner
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub fn centered(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x: x - w / 2,
            y: y - h / 2,
            w,
            h,
        }
    }

    // Right-most pixel (contained in the rect)
    pub fn right(&self) -> i32 {
        self.x + self.w - 1
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
