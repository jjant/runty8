mod rpg;
use rand::{thread_rng, Rng};
use rpg::currency::Currency;
use rpg::enemy::Enemy;
use rpg::entity::{Entity, EntityT, ShouldDestroy};
use rpg::inventory::Inventory;
use rpg::item::{DroppedItem, Item};
use rpg::player::Player;
use runty8::app::{ImportantApp, Right, WhichOne};
use runty8::ui::cursor::{self, Cursor};
use runty8::ui::{DrawFn, Element, Tree};
use runty8::Resources;
use runty8::{Event, Key, KeyState, KeyboardEvent};

fn main() {
    runty8::run_app::<GameState>("src/bin/game".to_owned());
}

struct GameState {
    player: Player,
    entities: Vec<Entity>,
    frames: usize,
    inventory_open: bool,
    inventory: Inventory,
    // mouse_x: i32,
    // mouse_y: i32,
    // mouse_clicked: bool,
    // highlighted_item: Option<usize>,
    // selected_item: Option<usize>,
    hovered_item: Option<usize>,
    keys: Keys,
    cursor: cursor::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Msg {
    Tick,
    ToggleInventory,
    KeyEvent { key_event: KeyboardEvent },
    HoveredItem(usize),
    UnHoveredItem(usize),
    RerollItem,
    Attack,
}
use Msg::*;

impl WhichOne for GameState {
    type Which = Right;
}

pub struct Keys {
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
        let mut snail = Enemy::snail(70, 70);
        snail.vx = 0;

        let entities = vec![
            // Entity::from(snail),
            // Entity::from(Enemy::mage(20, 80)),
            Entity::from(Enemy::mage(20, 100)),
            Entity::from(DroppedItem::new(Item::bde_staff(), 80, 60)),
        ];

        Self {
            player: Player::new(),
            entities,
            frames: 0,
            inventory_open: false,
            inventory: Inventory::new(),
            keys: Keys::new(),
            cursor: cursor::State::new(),
            hovered_item: None,
        }
    }

    fn update(&mut self, msg: &Msg, _: &mut Resources) {
        match *msg {
            ToggleInventory => {
                self.inventory_open = !self.inventory_open;
            }
            KeyEvent { key_event } => self.keys.update(key_event),
            Tick => {
                self.frames += 1;

                if self.frames % 15 == 0 {
                    let item = DroppedItem::new(
                        Item::bde_staff(),
                        thread_rng().gen_range(0..128),
                        thread_rng().gen_range(0..128),
                    );
                    self.entities.push(Entity::from(item))
                }

                self.player
                    .update(&self.keys, &mut self.inventory, self.entities.iter_mut());

                let mut new_entities = vec![];
                self.entities.retain_mut(|entity| {
                    let mut update_action = entity.update();

                    new_entities.append(&mut update_action.entities);

                    update_action.should_destroy == ShouldDestroy::No
                });
                self.entities.append(&mut new_entities)
                // self.entities.iter_mut().for_each(|entity| entity.update());
            }
            HoveredItem(index) => self.hovered_item = Some(index),
            UnHoveredItem(item) => {
                if self.hovered_item == Some(item) {
                    self.hovered_item = None;
                }
            }
            RerollItem => {
                self.orb_hovered(Currency::Blessed);
            }
            Attack => self.player.attack(),
        }
    }

    fn view(&mut self, _: &Resources) -> Element<'_, Self::Msg> {
        Tree::new()
            .push(DrawFn::new(|draw| {
                draw.cls();
                self.player.draw(draw, self.frames);
            }))
            .push(view_entities(&self.entities))
            .push(if self.inventory_open {
                self.inventory.view(self.hovered_item)
            } else {
                Tree::new().into()
            })
            // .push(tooltip)
            .push(Cursor::new(&mut self.cursor))
            .into()
    }

    fn subscriptions(&self, event: &Event) -> Option<Self::Msg> {
        match *event {
            Event::Keyboard(KeyboardEvent {
                key: Key::C,
                state: KeyState::Down,
            }) => Some(ToggleInventory),
            Event::Keyboard(KeyboardEvent {
                key: Key::X,
                state: KeyState::Down,
            }) => Some(RerollItem),
            Event::Keyboard(KeyboardEvent {
                key: Key::J,
                state: KeyState::Down,
            }) => Some(Attack),
            Event::Keyboard(key_event) => Some(KeyEvent { key_event }),
            Event::Mouse(_) => None,
            Event::Tick { .. } => Some(Tick),
        }
    }
}

impl GameState {
    fn orb_hovered(&mut self, orb: Currency) -> Option<()> {
        let hovered_item = self.hovered_item?;
        let item = self.inventory.get_mut(hovered_item)?;
        orb.apply(item);

        Some(())
    }
}
fn view_entities(entities: &[Entity]) -> Element<'_, Msg> {
    Tree::with_children(entities.iter().map(|entity| entity.view()).collect()).into()
}
