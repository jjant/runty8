use runty8::{self, App, Button, Color, DrawContext, State};

fn main() {
    runty8::run_app::<GameState>();
}

struct GameState {
    player: Player,
    inventory_open: bool,
    inventory: Inventory,
    mouse_x: i32,
    mouse_y: i32,
    highlighted_item: Option<usize>,
}
enum Action {
    HighlightItem(usize),
}

use Action::*;

impl App for GameState {
    type Action = Action;

    fn init() -> Self {
        Self {
            mouse_x: 64,
            mouse_y: 64,
            player: Player::new(),
            inventory_open: true,
            inventory: Inventory::new(),
            highlighted_item: None,
        }
    }

    fn update(&mut self, state: &State, actions: &[Action]) {
        let dx = state.btn(Button::Right) as i32 - state.btn(Button::Left) as i32;
        let dy = state.btn(Button::Down) as i32 - state.btn(Button::Up) as i32;

        self.mouse_x = state.mouse_x;
        self.mouse_y = state.mouse_y;

        self.player.x += dx;
        self.player.y += dy;

        if state.btnp(Button::C) {
            self.inventory_open = !self.inventory_open;
        }

        self.highlighted_item = None;
        for action in actions {
            match action {
                &HighlightItem(index) => self.highlighted_item = Some(index),
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
                draw_context,
                (self.mouse_x, self.mouse_y),
                HighlightItem,
            ));
        }

        if let Some(item) = self
            .highlighted_item
            .and_then(|index| self.inventory.items.get(index))
        {
            item.draw_tooltip(draw_context)
        }

        draw_context.spr(56, self.mouse_x, self.mouse_y);

        actions
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
    items: Vec<Item>,
}

impl Inventory {
    const START_X: i32 = 64 + 1;
    const START_Y: i32 = 64;
    fn new() -> Self {
        Self {
            items: vec![
                Item {
                    name: "SWORD",
                    description: "MELEE ATTACK",
                    sprite: 48,
                },
                Item {
                    name: "STAFF",
                    description: "CASTS SPELLS",
                    sprite: 51,
                },
            ],
        }
    }

    fn draw(
        &self,
        draw_context: &mut DrawContext,
        mouse_pos: (i32, i32),
        on_hover: impl Fn(usize) -> Action,
    ) -> Vec<Action> {
        let mut actions = vec![];
        draw_context.rectfill(64, 0, 128, 128, 6);

        for x_index in 0..5 {
            for y_index in 0..5 {
                let x = Self::START_X + x_index * 12;
                let y = Self::START_Y + y_index * 12;
                draw_context.rectfill(x, y, x + 9, y + 9, 7);

                Self::item_rect(x_index as usize, y_index as usize).fill(draw_context, 5);

                let index = (x_index + y_index * 5) as usize;
                if let Some(item) = self.items.get(index) {
                    if let Some(action) =
                        item.draw_slot(draw_context, x, y, mouse_pos, on_hover(index))
                    {
                        actions.push(action);
                    }
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
}

#[derive(Debug, Clone)]
struct Item {
    name: &'static str,
    description: &'static str,
    sprite: u8,
}

impl Item {
    fn draw_slot(
        &self,
        draw_context: &mut DrawContext,
        x: i32,
        y: i32,
        mouse_pos: (i32, i32),
        on_hover: Action,
    ) -> Option<Action> {
        draw_context.print(self.name, x, y, 8);
        draw_context.spr(self.sprite as usize, x + 1, y + 1);

        let sprite_rect = Rect {
            x: x + 1,
            y: y + 1,
            width: 8,
            height: 8,
        };

        if sprite_rect.contains(mouse_pos.0, mouse_pos.1) {
            Some(on_hover)
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
        let text_y = 20;
        let sprite_x = 64;
        let sprite_y = 38;

        draw_context.rect(x - w, y - h, x + w, y + h, 4);
        draw_context.rectfill(x + 1 - w, y + 1 - h, x - 1 + w, y - 1 + h, 7);
        draw_context.rectfill(x + 2 - w, y + 2 - h, x - 2 + w, y - 2 + h, 5);
        draw_context.print(self.name, text_x, text_y, 7);
        draw_context.print(self.description, text_x, text_y + 8, 7);
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
