use runty8::{
    colors,
    ui::{button, button::Button, DrawFn, Element, Tree},
};

use crate::Msg;

use super::{
    currency::Currency,
    item::{DroppedItem, Item, ItemType},
};

#[derive(Clone)]
pub struct ItemSlot {
    item: Option<Item>,
}

impl ItemSlot {
    fn new(item: Item) -> Self {
        Self { item: Some(item) }
    }

    fn empty() -> Self {
        Self { item: None }
    }

    pub fn view<'a>(
        &'a self,
        button: &'a mut button::State,
        x: i32,
        y: i32,
        index: usize,
    ) -> Element<'a, Msg> {
        const WIDTH: i32 = Item::WIDTH as i32;
        const HEIGHT: i32 = Item::HEIGHT as i32;

        let item_view = self
            .item
            .as_ref()
            .map(|item| item.view())
            .unwrap_or_else(|| Tree::new().into());

        Button::new(
            x,
            y,
            WIDTH,
            HEIGHT,
            None,
            button,
            Tree::new()
                .push(DrawFn::new(move |draw| {
                    // top
                    draw.line(1, 0, WIDTH - 2, 0, colors::LAVENDER);
                    // bottom
                    draw.line(1, HEIGHT - 1, WIDTH - 2, HEIGHT - 1, colors::LAVENDER);
                    // left
                    draw.line(0, 1, 0, HEIGHT - 2, colors::LAVENDER);
                    // right
                    draw.line(WIDTH - 1, 1, WIDTH - 1, HEIGHT - 2, colors::LAVENDER);

                    draw.rectfill(1, 1, WIDTH - 2, HEIGHT - 2, colors::LIGHT_GREY);
                }))
                .push(item_view),
        )
        .on_hover(Msg::HoveredItem(index))
        .on_leave(Msg::UnHoveredItem(index))
        .into()
    }

    fn fill(&mut self, item: Item) -> Option<Item> {
        match self.item {
            Some(_) => Some(item),
            None => {
                self.item = Some(item);
                None
            }
        }
    }
}

pub struct Inventory {
    items: Box<[ItemSlot]>,
    buttons: Vec<button::State>,
}

impl Inventory {
    const NUM_ITEMS: usize = 16;

    pub fn new() -> Self {
        let mut items = vec![ItemSlot::empty(); Self::NUM_ITEMS];
        items[0] = ItemSlot::new(Item::bde_staff());
        items[1] = ItemSlot::new(Item {
            name: "CHAOS ORB".to_owned(),
            sprite: 48,
            item_type: ItemType::Currency(Currency::Chaos),
        });
        items[2] = ItemSlot::new(Item {
            name: "BLESSED ORB".to_owned(),
            sprite: 49,
            item_type: ItemType::Currency(Currency::Blessed),
        });

        Self {
            items: items.into_boxed_slice(),
            buttons: vec![button::State::new(); Self::NUM_ITEMS],
        }
    }

    #[allow(dead_code)]
    fn get(&self, index: usize) -> Option<&Item> {
        let item = &self.items[index].item;

        item.as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Item> {
        let item = &mut self.items[index].item;

        item.as_mut()
    }

    pub fn view(&mut self, hovered_item: Option<usize>) -> Element<'_, Msg> {
        const BASE_X: i32 = 64;

        let weapon_slot = DrawFn::new(|draw| {
            let x = BASE_X + 3;
            let y = 20;

            draw.spr(6, x, y);
            draw.spr(7, x + 8, y);
            draw.spr(22, x, y + 8);
            draw.spr(23, x + 8, y + 8);
        });

        let items = &self.items;
        let tooltip = hovered_item
            .and_then(|item_index| items.get(item_index))
            .and_then(|slot| slot.item.as_ref())
            .map(|item| item.view_tooltip(20, 10))
            .unwrap_or_else(|| Tree::new().into());

        Tree::new()
            .push(DrawFn::new(|draw| {
                draw.rectfill(BASE_X, 0, 128, 128, colors::BROWN)
            }))
            .push(
                self.buttons
                    .iter_mut()
                    .zip(items.iter())
                    .enumerate()
                    .map(|(index, (button, item_slot))| {
                        const ITEMS_PER_ROW: usize = 4;
                        // TODO: Center
                        const OFFSET_X: usize = 5;

                        let x = OFFSET_X
                            + BASE_X as usize
                            + (index % ITEMS_PER_ROW) * (Item::WIDTH + 2);
                        let y = BASE_X as usize + (index / ITEMS_PER_ROW) * (Item::HEIGHT + 2);

                        item_slot.view(button, x as i32, y as i32, index)
                    })
                    .collect::<Vec<Element<'_, Msg>>>(),
            )
            .push(weapon_slot)
            .push(tooltip)
            .into()
    }

    pub fn push(&mut self, mut item: Item) -> Option<Item> {
        for slot in self.items.iter_mut() {
            match slot.fill(item) {
                None => return None,
                Some(new_item) => {
                    item = new_item;
                }
            }
        }

        Some(item)
    }

    pub fn push_dropped_item(&mut self, dropped_item: &mut DroppedItem) {
        if let Some(item) = dropped_item.destroy() {
            match self.push(item) {
                None => {}
                Some(item) => {
                    // Ugly hack
                    dropped_item.undestroy(item)
                }
            }
        }
    }
}
