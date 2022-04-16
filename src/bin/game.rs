use runty8::{self, App, Button, Color, DrawContext, State};
use std::fmt;

fn main() {
    runty8::run_app::<GameState>();
}

struct GameState {
    player: Player,
    inventory_open: bool,
    inventory: Inventory,
    mouse_x: i32,
    mouse_y: i32,
    mouse_clicked: bool,
    highlighted_item: Option<usize>,
    selected_item: Option<usize>,
}
enum Action {
    HighlightItem(usize),
    SelectedItem(usize),
    MovedSelectedItem { new_index: usize },
}

use Action::*;

impl App for GameState {
    type Action = Action;

    fn init() -> Self {
        Self {
            mouse_x: 64,
            mouse_y: 64,
            mouse_clicked: false,
            player: Player::new(),
            inventory_open: true,
            inventory: Inventory::new(),
            highlighted_item: None,
            selected_item: None,
        }
    }

    fn update(&mut self, state: &State, actions: &[Action]) {
        let dx = state.btn(Button::Right) as i32 - state.btn(Button::Left) as i32;
        let dy = state.btn(Button::Down) as i32 - state.btn(Button::Up) as i32;

        self.mouse_x = state.mouse_x;
        self.mouse_y = state.mouse_y;
        self.mouse_clicked = state.btnp(Button::Mouse);

        self.player.x += dx;
        self.player.y += dy;

        if state.btnp(Button::C) {
            self.inventory_open = !self.inventory_open;
        }

        self.highlighted_item = None;
        for action in actions {
            match *action {
                HighlightItem(index) => self.highlighted_item = Some(index),
                SelectedItem(index) => self.selected_item = Some(index),
                MovedSelectedItem { new_index } => {
                    if let Some(selected_item) = self.selected_item {
                        self.inventory.items.swap(selected_item, new_index);
                    }
                }
            }
        }
    }

    fn draw(&mut self, draw_context: &mut DrawContext) -> Vec<Action> {
        let mut actions = vec![];
        draw_context.palt(Some(0));
        draw_context.cls();
        self.player.draw(draw_context);

        if self.inventory_open {
            actions.append(&mut self.inventory.draw(
                self,
                draw_context,
                HighlightItem,
                SelectedItem,
            ));
        }

        if let Some(item) = self
            .highlighted_item
            .and_then(|index| self.inventory.items[index].item.as_ref())
        {
            item.draw_tooltip(draw_context)
        }

        self.draw_cursor(draw_context);

        actions
    }
}

impl GameState {
    fn draw_cursor(&self, draw_context: &mut DrawContext) {
        draw_context.spr(56, self.mouse_x, self.mouse_y);

        let mut draw_selected_item = || {
            let index = self.selected_item?;
            let item = self.inventory.items[index].item.as_ref()?;

            draw_context.spr(item.sprite as usize, self.mouse_x, self.mouse_y);

            Some(())
        };

        draw_selected_item();
    }

    fn selected_item(&self) -> Option<&Item> {
        self.inventory.items[self.selected_item?].item.as_ref()
    }
}
struct Player {
    x: i32,
    y: i32,
}

impl Player {
    fn new() -> Self {
        Self { x: 64, y: 64 }
    }

    fn draw(&self, draw_context: &mut DrawContext) {
        draw_context.spr(1, self.x, self.y)
    }
}

struct Inventory {
    items: Box<[ItemSlot]>,
}

#[derive(Debug, Clone, Copy)]
enum Attribute {
    Attack(i32),
    AttackSpeed(i32),
}
impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Attribute::Attack(attack) => write!(f, "{:+} ATTACK", attack),
            Attribute::AttackSpeed(attack_speed) => write!(f, "{:+} ATTACK SPEED", attack_speed),
        }
    }
}

impl Inventory {
    const START_X: i32 = 64 + 1;
    const START_Y: i32 = 64;

    const ROWS: usize = 5;
    const COLS: usize = 5;

    fn new() -> Self {
        use Attribute::*;

        let mut items = vec![ItemSlot { item: None }; Self::ROWS * Self::COLS].into_boxed_slice();
        items[0] = ItemSlot {
            item: Some(Item {
                name: "SWORD",
                description: "MELEE ATTACK",
                attributes: vec![Attack(1)],
                sprite: 48,
            }),
        };
        items[1] = ItemSlot {
            item: Some(Item {
                name: "STAFF",
                description: "CASTS SPELLS",
                attributes: vec![AttackSpeed(-1)],
                sprite: 51,
            }),
        };

        Self { items }
    }

    fn draw(
        &self,

        game_state: &GameState,
        draw_context: &mut DrawContext,
        on_hover: impl Fn(usize) -> Action,
        on_click: impl Fn(usize) -> Action,
    ) -> Vec<Action> {
        let mut actions = vec![];
        draw_context.rectfill(64, 0, 128, 128, 6);

        for x_index in 0..5 {
            for y_index in 0..5 {
                let index = Self::index(x_index, y_index);
                let item_slot = &self.items[index];

                if let Some(action) = item_slot.draw(
                    game_state,
                    draw_context,
                    x_index,
                    y_index,
                    on_hover(index),
                    on_click(index),
                ) {
                    actions.push(action);
                }
            }
        }
        actions
    }

    pub fn item_rect(x: usize, y: usize) -> Rect {
        let x = Self::START_X + x as i32 * 12;
        let y = Self::START_Y + y as i32 * 12;

        Rect {
            x: x + 1,
            y: y + 1,
            width: 8,
            height: 8,
        }
    }

    pub fn index(x_index: usize, y_index: usize) -> usize {
        x_index + y_index * 5
    }
}

#[derive(Clone)]
struct ItemSlot {
    item: Option<Item>,
}

impl ItemSlot {
    fn draw(
        &self,
        game_state: &GameState,
        draw_context: &mut DrawContext,
        x_index: usize,
        y_index: usize,
        on_hover: Action,
        on_click: Action,
    ) -> Option<Action> {
        let border = 1;
        let spacing = 1;
        // 12 = 8 (sprite) + 2 (border) + 2 (spacing)
        let x = Inventory::START_X + x_index as i32 * (8 + border * 2 + spacing * 2);
        let y = Inventory::START_Y + y_index as i32 * (8 + border * 2 + spacing * 2);
        draw_context.rectfill(x, y, x + 9, y + 9, 7);

        let slot_rect = Inventory::item_rect(x_index, y_index);
        slot_rect.fill(draw_context, 5);

        match &self.item {
            Some(item) => item.draw_slot(game_state, draw_context, x, y, on_hover, on_click),
            None => {
                let clicked = game_state.mouse_clicked
                    && slot_rect.contains(game_state.mouse_x, game_state.mouse_y);
                // move item to this slot
                if clicked {
                    let _ = game_state.selected_item?;

                    Some(MovedSelectedItem {
                        new_index: Inventory::index(x_index, y_index),
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Item {
    name: &'static str,
    description: &'static str,
    attributes: Vec<Attribute>,
    sprite: u8,
}

impl Item {
    fn draw_slot(
        &self,
        game_state: &GameState,
        draw_context: &mut DrawContext,
        x: i32,
        y: i32,
        on_hover: Action,
        on_click: Action,
    ) -> Option<Action> {
        draw_context.print(self.name, x, y, 8);
        draw_context.spr(self.sprite as usize, x + 1, y + 1);

        let sprite_rect = Rect {
            x: x + 1,
            y: y + 1,
            width: 8,
            height: 8,
        };
        let contained = sprite_rect.contains(game_state.mouse_x, game_state.mouse_y);
        if contained {
            if game_state.mouse_clicked {
                Some(on_click)
            } else {
                Some(on_hover)
            }
        } else {
            None
        }
    }

    fn draw_tooltip(&self, draw_context: &mut DrawContext) {
        let x = 64;
        let y = 30;
        let w = 30;
        let h = 20;

        let text_x = 40;
        let text_y = 16;
        let sprite_x = 64;
        let sprite_y = 38;

        draw_context.rect(x - w, y - h, x + w, y + h, 4);
        draw_context.rectfill(x + 1 - w, y + 1 - h, x - 1 + w, y - 1 + h, 7);
        draw_context.rectfill(x + 2 - w, y + 2 - h, x - 2 + w, y - 2 + h, 5);
        draw_context.print(self.name, text_x, text_y, 7);
        draw_context.print(self.description, text_x, text_y + 8, 7);

        for (index, attribute) in self.attributes.iter().enumerate() {
            let x = text_x;
            let y = text_y + 8 + (index + 1) as i32 * 8;
            draw_context.print(dbg!(&attribute.to_string()), x, y, 7);
        }
        draw_context.spr(self.sprite as usize, sprite_x, sprite_y);
    }
}

struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    pub fn contains(&self, x: i32, y: i32) -> bool {
        let contains_x = x >= self.x && x < self.x + self.width;
        let contains_y = y >= self.y && y < self.y + self.height;

        contains_x && contains_y
    }

    pub fn fill(&self, draw_context: &mut DrawContext, color: Color) {
        draw_context.rectfill(
            self.x,
            self.y,
            self.x + self.width - 1,
            self.y + self.height - 1,
            color,
        )
    }
}
