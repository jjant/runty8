use runty8::{
    runtime::draw_context::{colors, DrawContext},
    ui::{DrawFn, Element},
};

use crate::{rpg::rect::Rect, Msg};

pub struct Enemy {
    x: i32,
    y: i32,
    pub vx: i32,
    pub speed_x: i32,
    vy: i32,
    hp: i32,
    max_hp: i32,
    sprite: usize,
    flash_timer: i32,
    damage: i32,
    damage_counter: f32,
    hitbox: Rect,
    // https://gamedev.stackexchange.com/questions/40607/need-efficient-way-to-keep-enemy-from-getting-hit-multiple-times-by-same-source
    previous_frame_colliding: bool,
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
            hitbox: Rect::new(0, 0, 10, 10),
            previous_frame_colliding: false,
        }
    }

    pub fn mage(x: i32, y: i32) -> Self {
        Self::new(x, y, 57)
    }

    pub fn snail(x: i32, y: i32) -> Self {
        Self::new(x, y, 59)
    }

    pub fn handle_incoming_attack(&mut self, damage: i32, colliding: bool) {
        // This will need to be revisited many things can damage the same entity at once.
        if colliding && !self.previous_frame_colliding {
            //trigger hit with otherObject
            self.take_damage(damage);
        }
        self.previous_frame_colliding = colliding;
    }

    pub fn take_damage(&mut self, damage: i32) {
        let new_hp = i32::max(self.hp - damage, 0);
        let actual_damage = self.hp - new_hp;

        self.hp = new_hp;

        self.flash_timer += actual_damage * 15;
        self.damage += actual_damage;
        self.damage_counter += actual_damage as f32;
    }

    pub fn hitbox(&self) -> Rect {
        self.hitbox.translate(self.x, self.y)
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
        self.hitbox().outline(draw, 8);
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
            containing_rect.left() + 1,
            containing_rect.top() + 1,
            filled_width,
            BASE_HEIGHT,
        );
        current_hp_rect.fill(draw, colors::RED);

        let percentage_damage = self.damage_counter / self.max_hp as f32;
        let damage_width = (percentage_damage * BASE_WIDTH as f32).round() as i32;
        Rect::new(
            current_hp_rect.right() + 1,
            containing_rect.top() + 1,
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
