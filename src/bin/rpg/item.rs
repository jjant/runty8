use crate::{rpg::currency::Currency, Msg};
use runty8::{
    runtime::draw_context::colors,
    ui::{
        button::{self, Button},
        DrawFn, Element,
    },
};
use ItemType::*;

use super::modifier::{ImplicitModifier, Modifier};

#[derive(Clone)]
pub struct Item {
    name: String,
    sprite: usize,
    item_type: ItemType,
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

    pub fn view<'a>(
        &'a self,
        button: &'a mut button::State,
        x: i32,
        y: i32,
        index: usize,
    ) -> Element<'a, Msg> {
        let sprite = self.sprite;

        Button::new(
            x,
            y,
            Self::WIDTH as i32,
            Self::HEIGHT as i32,
            None,
            button,
            DrawFn::new(move |draw| {
                let w = Self::WIDTH as i32;
                let h = Self::HEIGHT as i32;

                // top
                draw.line(1, 0, w - 2, 0, colors::LAVENDER);
                // bottom
                draw.line(1, h - 1, w - 2, h - 1, colors::LAVENDER);
                // left
                draw.line(0, 1, 0, h - 2, colors::LAVENDER);
                // right
                draw.line(w - 1, 1, w - 1, h - 2, colors::LAVENDER);

                draw.rectfill(1, 1, w - 2, h - 2, colors::LIGHT_GREY);

                draw.spr(sprite, 2, 2);
            }),
        )
        .on_hover(Msg::HoveredItem(index))
        .on_leave(Msg::UnHoveredItem(index))
        .into()
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
