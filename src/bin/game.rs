use runty8::app::{ImportantApp, Right, WhichOne};
use runty8::runtime::draw_context::DrawContext;
use runty8::screen::Resources;
use runty8::ui::button::Button;
use runty8::ui::cursor::{self, Cursor};
use runty8::ui::text::Text;
use runty8::ui::{button, DrawFn, Element, Tree};
use runty8::{Event, Key, KeyState, KeyboardEvent};

fn main() {
    runty8::run_app::<GameState>("src/bin/game".to_owned());
}

struct GameState {
    player: Player,
    frames: usize,
    inventory_open: bool,
    inventory: Inventory,
    // mouse_x: i32,
    // mouse_y: i32,
    // mouse_clicked: bool,
    // highlighted_item: Option<usize>,
    // selected_item: Option<usize>,
    keys: Keys,
    cursor: cursor::State,
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    Tick,
    ToggleInventory,
    KeyEvent { key_event: KeyboardEvent },
}
use Msg::*;

impl WhichOne for GameState {
    type Which = Right;
}

struct Keys {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}
impl Keys {
    fn new() -> Self {
        Self {
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }

    fn update(&mut self, key_event: KeyboardEvent) {
        match key_event {
            KeyboardEvent { key: Key::W, state } => {
                self.up = state == KeyState::Down;
            }
            KeyboardEvent { key: Key::A, state } => {
                self.left = state == KeyState::Down;
            }
            KeyboardEvent { key: Key::S, state } => {
                self.down = state == KeyState::Down;
            }
            KeyboardEvent { key: Key::D, state } => {
                self.right = state == KeyState::Down;
            }
            _ => {}
        }
    }
}
impl ImportantApp for GameState {
    type Msg = Msg;

    fn init() -> Self {
        Self {
            player: Player::new(),
            frames: 0,
            inventory_open: false,
            inventory: Inventory::new(),
            keys: Keys::new(),
            cursor: cursor::State::new(),
        }
    }

    fn update(&mut self, msg: &Msg, _: &mut Resources) {
        match msg {
            ToggleInventory => {
                self.inventory_open = !self.inventory_open;
            }
            &KeyEvent { key_event } => self.keys.update(key_event),
            Tick => {
                self.frames += 1;
                self.player.update(&self.keys);
            }
        }
    }

    fn view(&mut self, _: &Resources) -> Element<'_, Self::Msg> {
        Tree::new()
            .push(DrawFn::new(|draw| {
                draw.cls();
                self.player.draw(draw, self.frames);
            }))
            .push(if self.inventory_open {
                self.inventory.view()
            } else {
                Tree::new().into()
            })
            .push(Cursor::new(&mut self.cursor))
            .into()
    }

    fn subscriptions(&self, event: &Event) -> Option<Self::Msg> {
        match event {
            Event::Keyboard(KeyboardEvent {
                key: Key::C,
                state: KeyState::Down,
            }) => Some(ToggleInventory),
            &Event::Keyboard(key_event) => Some(KeyEvent { key_event }),
            Event::Mouse(_) => None,
            Event::Tick { .. } => Some(Tick),
        }
    }
}

#[derive(Clone)]
struct Item {
    name: String,
}

impl Item {
    const HEIGHT: i32 = 10;
    const WIDTH: i32 = 10;

    fn view<'a>(&'a self, button: &'a mut button::State, x: i32, y: i32) -> Element<'a, Msg> {
        Button::new(
            x,
            y,
            Self::WIDTH,
            Self::HEIGHT,
            None,
            button,
            Text::new(&self.name, 0, 0, 7),
        )
        .into()
    }
}

struct Inventory {
    items: Vec<Item>,
    buttons: Vec<button::State>,
}

impl Inventory {
    const NUM_ITEMS: usize = 10;
    fn new() -> Self {
        Self {
            items: vec![
                Item {
                    name: "BDE STAFF".to_owned()
                };
                Self::NUM_ITEMS
            ],
            buttons: vec![button::State::new(); Self::NUM_ITEMS],
        }
    }

    fn view(&mut self) -> Element<'_, Msg> {
        Tree::new()
            .push(DrawFn::new(|draw| draw.rectfill(64, 0, 128, 128, 6)))
            .push(
                self.buttons
                    .iter_mut()
                    .zip(self.items.iter())
                    .enumerate()
                    .map(|(index, (button, item))| {
                        let rows = 2;
                        let x = 30 + (index as i32 % rows) * (Item::WIDTH + 2);
                        let y = 30 + (index as i32 / rows) * (Item::HEIGHT + 2);

                        item.view(button, x, y)
                    })
                    .collect::<Vec<Element<'_, Msg>>>(),
            )
            .into()
    }
}

// enum Action {
//     HighlightItem(usize),
//     SelectedItem(usize),
//     MovedSelectedItem { new_index: usize },
//     AppliedOrb { orb_index: usize, item_index: usize },
// }

// use Action::*;

// impl ElmApp for GameState {
//     type Action = Action;

//     fn init() -> Self {
//         Self {
//             mouse_x: 64,
//             mouse_y: 64,
//             mouse_clicked: false,
//             player: Player::new(),
//             inventory_open: true,
//             inventory: Inventory::new(),
//             highlighted_item: None,
//             selected_item: None,
//         }
//     }

//     fn update(&mut self, state: &State, actions: &[Action]) {
//         let dx = state.btn(Button::Right) as i32 - state.btn(Button::Left) as i32;
//         let dy = state.btn(Button::Down) as i32 - state.btn(Button::Up) as i32;

//         self.mouse_x = state.mouse_x;
//         self.mouse_y = state.mouse_y;
//         self.mouse_clicked = state.btnp(Button::Mouse);

//         self.player.x += dx;
//         self.player.y += dy;

//         if state.btnp(Button::C) {
//             self.inventory_open = !self.inventory_open;
//         }

//         self.highlighted_item = None;
//         for action in actions {
//             match *action {
//                 HighlightItem(index) => self.highlighted_item = Some(index),
//                 SelectedItem(index) => self.selected_item = Some(index),
//                 MovedSelectedItem { new_index } => {
//                     if let Some(selected_item) = self.selected_item {
//                         self.inventory.items.swap(selected_item, new_index);
//                     }
//                 }
//                 AppliedOrb {
//                     orb_index,
//                     item_index,
//                 } => {
//                     let orb_was_present = self.inventory.remove(orb_index).is_some();

//                     if orb_was_present {
//                         if let Some(item) = &mut self.inventory.items[item_index].item {
//                             item.apply_power_orb()
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     fn draw(&mut self, draw_context: &mut DrawContext) -> Vec<Action> {
//         let mut actions = vec![];
//         draw_context.palt(Some(0));
//         draw_context.cls();
//         self.player.draw(draw_context);

//         if self.inventory_open {
//             actions.append(&mut self.inventory.draw(
//                 self,
//                 draw_context,
//                 HighlightItem,
//                 SelectedItem,
//             ));
//         }

//         if let Some(item) = self
//             .highlighted_item
//             .and_then(|index| self.inventory.items[index].item.as_ref())
//         {
//             item.draw_tooltip(draw_context)
//         }

//         self.draw_cursor(draw_context);

//         actions
//     }
// }

// impl GameState {
//     fn draw_cursor(&self, draw_context: &mut DrawContext) {
//         draw_context.spr(56, self.mouse_x, self.mouse_y);

//         let mut draw_selected_item = || {
//             let index = self.selected_item?;
//             let item = self.inventory.items[index].item.as_ref()?;

//             draw_context.spr(item.sprite as usize, self.mouse_x, self.mouse_y);

//             Some(())
//         };

//         draw_selected_item();
//     }

//     fn selected_item(&self) -> Option<(usize, &Item)> {
//         let index = self.selected_item?;
//         let item = self.inventory.items[index].item.as_ref()?;

//         Some((index, item))
//     }
// }
struct Player {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 64,
            y: 64,
            vx: 0,
            vy: 0,
        }
    }

    fn update(&mut self, keys: &Keys) {
        self.vx = keys.right as i32 - keys.left as i32;
        self.vy = keys.down as i32 - keys.up as i32;

        self.x += self.vx;
        self.x = clamp(self.x, 0, 120);
        self.y += self.vy;
        self.y = clamp(self.y, 0, 120);
    }

    fn draw(&self, draw_context: &mut DrawContext, frames: usize) {
        const BASE_SPR: usize = 1;
        const NUM_SPR: usize = 2;
        let sprite = if self.vx != 0 {
            animate(BASE_SPR, NUM_SPR, 5, frames)
        } else {
            BASE_SPR
        };

        draw_context.spr(sprite, self.x, self.y);
    }
}

fn animate(base: usize, count: usize, every_num_frames: usize, t: usize) -> usize {
    base + (t / every_num_frames) % count
}

// #[derive(Debug, Clone, Copy)]
// enum Attribute {
//     Attack(i32),
//     AttackSpeed(i32),
// }
// impl fmt::Display for Attribute {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Attribute::Attack(attack) => write!(f, "{:+} ATTACK", attack),
//             Attribute::AttackSpeed(attack_speed) => write!(f, "{:+} ATTACK SPEED", attack_speed),
//         }
//     }
// }

// struct Inventory {
//     items: Box<[ItemSlot]>,
// }

// impl Inventory {
//     const START_X: i32 = 64 + 1;
//     const START_Y: i32 = 64;

//     const ROWS: usize = 5;
//     const COLS: usize = 5;

//     fn new() -> Self {
//         use Attribute::*;

//         let mut items = vec![ItemSlot::empty(); Self::ROWS * Self::COLS].into_boxed_slice();
//         items[0] = ItemSlot::new(Item {
//             name: "SWORD",
//             description: "MELEE ATTACK",
//             attributes: vec![Attack(1)],
//             tags: vec![],
//             sprite: 48,
//         });
//         items[1] = ItemSlot::new(Item {
//             name: "STAFF",
//             description: "CASTS SPELLS",
//             attributes: vec![AttackSpeed(-1)],
//             tags: vec![],
//             sprite: 51,
//         });

//         items[14] = ItemSlot::new(Item {
//             name: "POWER GEM",
//             description: "RND ATTRS",
//             attributes: vec![],
//             tags: vec![Tag::RightClickable],
//             sprite: 32,
//         });

//         for i in 15..25 {
//             items[i] = items[14].clone();
//         }

//         Self { items }
//     }

//     fn draw(
//         &self,

//         game_state: &GameState,
//         draw_context: &mut DrawContext,
//         on_hover: impl Fn(usize) -> Action,
//         on_click: impl Fn(usize) -> Action,
//     ) -> Vec<Action> {
//         let mut actions = vec![];
//         draw_context.rectfill(64, 0, 128, 128, 6);

//         for x_index in 0..5 {
//             for y_index in 0..5 {
//                 let index = Self::index(x_index, y_index);
//                 let item_slot = &self.items[index];

//                 if let Some(action) = item_slot.draw(
//                     game_state,
//                     draw_context,
//                     x_index,
//                     y_index,
//                     on_hover(index),
//                     on_click(index),
//                 ) {
//                     actions.push(action);
//                 }
//             }
//         }
//         actions
//     }

//     pub fn item_rect(x: usize, y: usize) -> Rect {
//         let x = Self::START_X + x as i32 * 12;
//         let y = Self::START_Y + y as i32 * 12;

//         Rect {
//             x: x + 1,
//             y: y + 1,
//             width: 8,
//             height: 8,
//         }
//     }

//     pub fn index(x_index: usize, y_index: usize) -> usize {
//         x_index + y_index * 5
//     }

//     fn remove(&mut self, index: usize) -> Option<Item> {
//         self.items[index].take()
//     }
// }

// #[derive(Clone)]
// struct ItemSlot {
//     item: Option<Item>,
// }

// impl ItemSlot {
//     fn empty() -> Self {
//         Self { item: None }
//     }

//     fn new(item: Item) -> Self {
//         Self { item: Some(item) }
//     }

//     fn draw(
//         &self,
//         game_state: &GameState,
//         draw_context: &mut DrawContext,
//         x_index: usize,
//         y_index: usize,
//         on_hover: Action,
//         on_click: Action,
//     ) -> Option<Action> {
//         let border = 1;
//         let spacing = 1;
//         let x = Inventory::START_X + x_index as i32 * (8 + border * 2 + spacing * 2);
//         let y = Inventory::START_Y + y_index as i32 * (8 + border * 2 + spacing * 2);
//         draw_context.rectfill(x, y, x + 9, y + 9, 7);

//         let slot_rect = Inventory::item_rect(x_index, y_index);
//         slot_rect.fill(draw_context, 5);

//         let slot_clicked =
//             game_state.mouse_clicked && slot_rect.contains(game_state.mouse_x, game_state.mouse_y);

//         match &self.item {
//             Some(item) => {
//                 let e = item.draw_slot(game_state, draw_context, x, y, on_hover, on_click);

//                 if slot_clicked {
//                     if let Some((orb_index, selected_item)) = game_state.selected_item() {
//                         if selected_item.tags.contains(&Tag::RightClickable) {
//                             return Some(Action::AppliedOrb {
//                                 orb_index,
//                                 item_index: Inventory::index(x_index, y_index),
//                             });
//                         }
//                     }
//                 }

//                 e
//             }
//             None => {
//                 // move item to this slot
//                 if slot_clicked {
//                     let _ = game_state.selected_item?;

//                     Some(MovedSelectedItem {
//                         new_index: Inventory::index(x_index, y_index),
//                     })
//                 } else {
//                     None
//                 }
//             }
//         }
//     }

//     pub fn take(&mut self) -> Option<Item> {
//         self.item.take()
//     }
// }

// #[derive(Debug, Clone)]
// struct Item {
//     name: &'static str,
//     description: &'static str,
//     attributes: Vec<Attribute>,
//     tags: Vec<Tag>,
//     sprite: u8,
// }

// #[derive(Debug, Clone, PartialEq)]
// enum Tag {
//     RightClickable,
// }

// impl Item {
//     fn apply_power_orb(&mut self) {
//         for modifier in self.attributes.iter_mut() {
//             let new_modifier = match modifier {
//                 Attribute::Attack(_) => {
//                     let v = rand::thread_rng().gen_range(-2..=2);
//                     Attribute::Attack(v)
//                 }
//                 Attribute::AttackSpeed(_) => {
//                     let v = rand::thread_rng().gen_range(-2..=2);
//                     Attribute::AttackSpeed(v)
//                 }
//             };

//             *modifier = dbg!(new_modifier);
//         }
//     }

//     fn draw_slot(
//         &self,
//         game_state: &GameState,
//         draw_context: &mut DrawContext,
//         x: i32,
//         y: i32,
//         on_hover: Action,
//         on_click: Action,
//     ) -> Option<Action> {
//         draw_context.print(self.name, x, y, 8);
//         draw_context.spr(self.sprite as usize, x + 1, y + 1);

//         let sprite_rect = Rect {
//             x: x + 1,
//             y: y + 1,
//             width: 8,
//             height: 8,
//         };
//         let contained = sprite_rect.contains(game_state.mouse_x, game_state.mouse_y);
//         if contained {
//             if game_state.mouse_clicked {
//                 Some(on_click)
//             } else {
//                 Some(on_hover)
//             }
//         } else {
//             None
//         }
//     }

//     fn draw_tooltip(&self, draw_context: &mut DrawContext) {
//         let x = 64;
//         let y = 30;
//         let w = 30;
//         let h = 20;

//         let text_x = 40;
//         let text_y = 16;
//         let sprite_x = 64;
//         let sprite_y = 38;

//         draw_context.rect(x - w, y - h, x + w, y + h, 4);
//         draw_context.rectfill(x + 1 - w, y + 1 - h, x - 1 + w, y - 1 + h, 7);
//         draw_context.rectfill(x + 2 - w, y + 2 - h, x - 2 + w, y - 2 + h, 5);
//         draw_context.print(self.name, text_x, text_y, 7);
//         draw_context.print(self.description, text_x, text_y + 8, 7);

//         for (index, attribute) in self.attributes.iter().enumerate() {
//             let x = text_x;
//             let y = text_y + 8 + (index + 1) as i32 * 8;
//             draw_context.print(dbg!(&attribute.to_string()), x, y, 7);
//         }
//         draw_context.spr(self.sprite as usize, sprite_x, sprite_y);
//     }
// }

// struct Rect {
//     x: i32,
//     y: i32,
//     width: i32,
//     height: i32,
// }

// impl Rect {
//     pub fn contains(&self, x: i32, y: i32) -> bool {
//         let contains_x = x >= self.x && x < self.x + self.width;
//         let contains_y = y >= self.y && y < self.y + self.height;

//         contains_x && contains_y
//     }

//     pub fn fill(&self, draw_context: &mut DrawContext, color: Color) {
//         draw_context.rectfill(
//             self.x,
//             self.y,
//             self.x + self.width - 1,
//             self.y + self.height - 1,
//             color,
//         )
//     }
// }

fn clamp(val: i32, a: i32, b: i32) -> i32 {
    a.max(b.min(val))
}
