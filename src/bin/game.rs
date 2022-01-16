use runty8::{self, App, Button, DrawContext, State};

fn main() {
    runty8::run_app::<GameState>();
}

struct GameState {
    player: Player,
    inventory_open: bool,
    inventory: Inventory,
    mouse_x: i32,
    mouse_y: i32,
}

impl App for GameState {
    fn init() -> Self {
        Self {
            mouse_x: 64,
            mouse_y: 64,
            player: Player::new(),
            inventory_open: true,
            inventory: Inventory::new(),
        }
    }

    fn update(&mut self, state: &State) {
        let dx = state.btn(Button::Right) as i32 - state.btn(Button::Left) as i32;
        let dy = state.btn(Button::Down) as i32 - state.btn(Button::Up) as i32;

        self.mouse_x = state.mouse_x;
        self.mouse_y = state.mouse_y;

        self.player.x += dx;
        self.player.y += dy;
    }

    fn draw(&self, draw_context: &mut DrawContext) {
        draw_context.palt(Some(0));
        draw_context.cls();
        self.player.draw(draw_context);

        if self.inventory_open {
            self.inventory.draw(draw_context);
        }

        draw_context.spr(56, self.mouse_x, self.mouse_y);
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
    fn new() -> Self {
        Self {
            items: vec![
                Item {
                    name: "SWORD",
                    sprite: 48,
                },
                Item {
                    name: "STAFF",
                    sprite: 51,
                },
            ],
        }
    }

    fn draw(&self, draw_context: &mut DrawContext) {
        draw_context.rectfill(64, 0, 128, 128, 6);

        let start_x = 64 + 1;
        let start_y = 64;

        for x_index in 0..5 {
            for y_index in 0..5 {
                let x = start_x + x_index * 12;
                let y = start_y + y_index * 12;
                draw_context.rectfill(x, y, x + 9, y + 9, 7);
                draw_context.rectfill(x + 1, y + 1, x + 8, y + 8, 5);

                let index = (x_index + y_index * 5) as usize;
                if let Some(item) = self.items.get(index) {
                    draw_context.print(item.name, x, y, 8);
                    draw_context.spr(item.sprite as usize, x + 1, y + 1);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Item {
    name: &'static str,
    sprite: u8,
}
