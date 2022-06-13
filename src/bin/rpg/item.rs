use crate::{rpg::currency::Currency, Msg};
use runty8::{
    runtime::draw_context::colors,
    ui::{DrawFn, Element},
};
use ItemType::*;

use super::{
    entity::{EntityT, ShouldDestroy},
    modifier::{ImplicitModifier, Modifier},
};

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub sprite: usize,
    pub item_type: ItemType,
}

impl EntityT for Item {
    fn update(&mut self) -> ShouldDestroy {
        ShouldDestroy::No
    }

    fn view(&self) -> Element<'_, Msg> {
        DrawFn::new(|draw| {
            draw.spr(self.sprite, 64, 64);
        })
        .into()
    }
}

#[derive(Clone)]
pub struct Wearable {
    pub implicits: Vec<ImplicitModifier>,
    pub mods: Vec<Modifier>,
}

#[derive(Clone)]
pub enum ItemType {
    Wearable(Wearable),
    Currency(Currency),
}

impl Item {
    pub const HEIGHT: usize = 12;
    pub const WIDTH: usize = 12;

    pub fn bde_staff() -> Self {
        Self {
            name: "BDE STAFF".to_owned(),
            sprite: 51,
            item_type: Wearable(Wearable {
                implicits: vec![ImplicitModifier::AttackDamage { min: 2, max: 5 }],
                mods: vec![Modifier::Attack(32)],
            }),
        }
    }

    pub fn to_wearable_mut(&mut self) -> Option<&mut Wearable> {
        match &mut self.item_type {
            ItemType::Wearable(wearable) => Some(wearable),
            ItemType::Currency(_) => None,
        }
    }

    pub fn view(&self) -> Element<'_, Msg> {
        let sprite = self.sprite;

        DrawFn::new(move |draw| draw.spr(sprite, 2, 2)).into()
    }

    pub fn view_tooltip(&self, x: i32, y: i32) -> Element<'_, Msg> {
        let (implicits, mods): (&[ImplicitModifier], &[Modifier]) = match &self.item_type {
            ItemType::Wearable(Wearable { implicits, mods }) => (implicits, mods),
            ItemType::Currency(_) => (&[], &[]),
        };

        DrawFn::new(move |draw| {
            let implicits_height = 6 * implicits.len() as i32;
            let mod_height = 6 * mods.len() as i32;
            let height = 20 + implicits_height + mod_height;
            let end_x = 127 - x;
            let end_y = y + height - 1;

            draw.rectfill(x, y, end_x, end_y, 13);
            draw.rect(x + 1, y + 1, end_x - 1, end_y - 1, 7);
            draw.print(&self.name, x + 3, y + 3, 7);

            for (index, modifier) in implicits.iter().enumerate() {
                let x = x + 3;
                let y = y + 3 + (index as i32 + 1) * 8;

                draw.print(&modifier.to_string(), x, y, colors::ORANGE);
            }

            for (index, modifier) in mods.iter().enumerate() {
                let x = x + 3;
                let y = y + 3 + (index as i32 + 1) * 8 + implicits_height;

                draw.print(&modifier.to_string(), x, y, colors::WHITE);
            }

            draw.spr(self.sprite, end_x - 10, end_y - 10);
        })
        .into()
    }
}
