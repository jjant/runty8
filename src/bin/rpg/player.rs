use runty8::{colors, Pico8};

use crate::{rpg::animate, Keys};

use super::{clamp, entity::Entity, inventory::Inventory, rect::Rect};

pub struct Player {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
    attack_timer: i32,
    attack_damage: i32,
    hitbox: Rect,
}

impl Player {
    const ATTACK_FRAME_TIME: i32 = 3;
    const ATTACK_TIME: i32 = 3 * Self::ATTACK_FRAME_TIME;
    const LOCAL_ATTACK_HITBOX: Rect = Rect::new(4, 0, 8, 8);

    pub fn new() -> Self {
        Self {
            x: 64,
            y: 64,
            vx: 0,
            vy: 0,
            attack_timer: 0,
            attack_damage: 2,
            hitbox: Rect::new(1, 1, 4, 7),
        }
    }

    pub fn update<'a>(
        &mut self,
        keys: &Keys,
        inventory: &mut Inventory,
        entities: impl Iterator<Item = &'a mut Entity>,
    ) {
        self.update_entities(inventory, entities);

        if self.attack_timer > 0 {
            self.attack_timer -= 1;
        }

        self.vx = keys.right as i32 - keys.left as i32;
        self.vy = keys.down as i32 - keys.up as i32;

        self.x += self.vx;
        self.x = clamp(self.x, 0, 120);
        self.y += self.vy;
        self.y = clamp(self.y, 0, 120);
    }

    pub fn draw(&self, draw: &mut Pico8, frames: usize) {
        const BASE_SPR: usize = 1;
        const NUM_SPR: usize = 2;

        let sprite = if self.attack_timer > 0 {
            4
        } else if self.vx != 0 {
            animate(BASE_SPR, NUM_SPR, 5, frames)
        } else {
            BASE_SPR
        };

        draw.spr(sprite, self.x, self.y);

        if self.attack_timer > 0 {
            let t = (Self::ATTACK_TIME - self.attack_timer) as usize;
            let attack_sprite = animate(16, 3, Self::ATTACK_FRAME_TIME as usize, t);

            draw.spr(attack_sprite, self.x + 4, self.y);
            if let Some(hitbox) = self.attack_hitbox() {
                hitbox.outline(draw, 7)
            }
        }

        self.hitbox().outline(draw, colors::WHITE)
    }

    pub fn attack(&mut self) {
        if self.attack_timer > 0 {
            return;
        }

        self.attack_timer = Self::ATTACK_TIME;
    }

    pub fn update_entities<'a>(
        &self,
        inventory: &mut Inventory,
        entities: impl Iterator<Item = &'a mut Entity>,
    ) {
        for entity in entities {
            match entity {
                Entity::Enemy(enemy) => {
                    let colliding = self
                        .attack_hitbox()
                        .map(|hitbox| enemy.hitbox().intersects(hitbox))
                        .unwrap_or(false);

                    enemy.handle_incoming_attack(self.attack_damage, colliding);
                }
                Entity::DroppedItem(dropped_item) => {
                    let colliding = dropped_item.hitbox().intersects(self.hitbox());
                    if !colliding {
                        continue;
                    }
                    println!("Colliding with {:?}", dropped_item);

                    inventory.push_dropped_item(dropped_item);
                }
            }
        }
    }

    // World-space hitbox
    fn attack_hitbox(&self) -> Option<Rect> {
        if self.attack_timer > 0 {
            Some(Self::LOCAL_ATTACK_HITBOX.translate(self.x, self.y))
        } else {
            None
        }
    }

    pub fn hitbox(&self) -> Rect {
        self.hitbox.translate(self.x, self.y)
    }
}
