use rand::{thread_rng, Rng};

use runty8::{
    colors,
    ui::{DrawFn, Element},
    Pico8,
};

use crate::{rpg::rect::Rect, Msg};

use super::{
    entity::{Entity, EntityT, ShouldDestroy, UpdateAction},
    item::{DroppedItem, Item},
};

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
    death_timer: Option<i32>,
}

impl EntityT for Enemy {
    fn update(&mut self) -> UpdateAction {
        if self.x > 121 {
            self.vx = -self.speed_x;
        } else if self.x < 0 {
            self.vx = self.speed_x;
        }

        self.x += self.vx;
        self.y += self.vy;

        self.handle_damage_animation();
        self.handle_death()
    }

    fn view(&self) -> Element<'_, Msg> {
        DrawFn::new(move |draw| {
            self.view_sprite(draw);
            self.view_hp_bar(draw);
        })
        .into()
    }
}

impl Enemy {
    const DEATH_FRAMES: &'static [AnimationFrame<u8>] = &[
        AnimationFrame {
            value: 59,
            duration: 3,
        },
        AnimationFrame {
            value: 60,
            duration: 3,
        },
        AnimationFrame {
            value: 61,
            duration: 3,
        },
        AnimationFrame {
            value: 62,
            duration: 3,
        },
        AnimationFrame {
            value: 63,
            duration: 3,
        },
        AnimationFrame {
            value: 47,
            duration: 3,
        },
        AnimationFrame {
            value: 46,
            duration: 3,
        },
    ];

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
            death_timer: None,
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

    fn take_damage(&mut self, damage: i32) {
        if self.hp <= 0 {
            return;
        }

        let new_hp = i32::max(self.hp - damage, 0);
        let actual_damage = self.hp - new_hp;

        self.hp = new_hp;

        self.flash_timer += actual_damage * 15;
        self.damage += actual_damage;
        self.damage_counter += actual_damage as f32;

        if self.hp <= 0 {
            self.death_timer = Some(Self::death_duration());
        }
    }

    pub fn hitbox(&self) -> Rect {
        self.hitbox.translate(self.x, self.y)
    }

    fn view_sprite(&self, draw: &mut Pico8) {
        if self.death_timer.is_some() {
            // Don't animate damage flash if dying
        } else if self.flash_timer > 0 && self.flash_timer % 2 == 0 {
            draw.pal(3, 7);
            draw.pal(1, 10);
            draw.pal(6, 9);
        }
        draw.spr(self.sprite, self.x, self.y);
        draw.reset_pal();
        self.hitbox().outline(draw, 8);
    }

    fn view_hp_bar(&self, draw: &mut Pico8) {
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

    fn handle_death(&mut self) -> UpdateAction {
        if let Some(death_timer) = &mut self.death_timer {
            if *death_timer <= 0 {
                let item_drop = self.compute_drop();
                return UpdateAction {
                    should_destroy: ShouldDestroy::Yes,
                    entities: item_drop.into_iter().map(Entity::from).collect(),
                };
            }

            *death_timer -= 1;

            let frame = Self::death_duration() - *death_timer;
            self.sprite = Self::death_sprite(frame);
        }

        UpdateAction {
            should_destroy: ShouldDestroy::No,
            entities: vec![],
        }
    }

    fn death_sprite(frame: i32) -> usize {
        *AnimationFrame::get(Self::DEATH_FRAMES, frame) as usize
    }
    fn death_duration() -> i32 {
        AnimationFrame::duration(Self::DEATH_FRAMES)
    }

    // Simulate an item drop based on enemy "level"
    fn compute_drop(&self) -> Option<DroppedItem> {
        let should_drop = thread_rng().gen();

        if dbg!(should_drop) {
            let min_damage = i32::max(self.max_hp - thread_rng().gen_range(1..=3), 0);
            let max_damage = self.max_hp + thread_rng().gen_range(1..=3);

            let item = Item::weapon("BDE STAFF+".to_owned(), 51, min_damage, max_damage, vec![]);

            let dropped_item = DroppedItem::new(item, self.x, self.y);

            Some(dropped_item)
        } else {
            None
        }
    }
}

struct AnimationFrame<T> {
    duration: i32,
    value: T,
}
impl<T> AnimationFrame<T> {
    fn get(frames: &[AnimationFrame<T>], frame: i32) -> &T {
        let mut to_go = frame;

        for animation_frame in frames {
            to_go -= animation_frame.duration;
            if to_go <= 0 {
                return &animation_frame.value;
            }
        }

        &frames.last().unwrap().value
    }

    fn duration(frames: &[AnimationFrame<T>]) -> i32 {
        frames.iter().map(|frame| frame.duration).sum()
    }
}
