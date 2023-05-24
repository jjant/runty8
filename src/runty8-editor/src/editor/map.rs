use crate::ui::button::{self, Button};
use crate::ui::{DrawFn, Element, Tree};
use crate::util::vec2::{vec2, Vec2i};
use itertools::Itertools;
use runty8_core::{Event, InputEvent, Key, KeyState, KeyboardEvent, Map, MouseEvent, Sprite};
use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct Editor {
    buttons: Vec<button::State>,
    show_sprites_in_map: bool,
    hovered_tile: (usize, usize),
    mouse_position: Vec2i,
    camera: Vec2i,
    // TODO: Use a proper enum
    dragging: bool,
}

impl Editor {
    const FILLER_SPR: usize = 1;

    pub(crate) fn new() -> Self {
        Self {
            buttons: vec![button::State::new(); 144],
            show_sprites_in_map: true,
            hovered_tile: (0, 0),
            mouse_position: vec2(64, 64),
            camera: Vec2i::zero(),
            dragging: false,
        }
    }

    pub(crate) fn update(&mut self, msg: Msg) {
        match msg {
            Msg::MouseMove(mouse_position) => {
                let delta = self.mouse_position - mouse_position;

                // println!("[Map Editor] Mouse delta: {delta:?}");

                self.mouse_position = mouse_position;
                if self.dragging {
                    self.camera = self.camera - delta;
                }
            }
            Msg::SetDragging(dragging) => {
                self.dragging = dragging;
            }
            Msg::SwitchMapMode => {
                self.show_sprites_in_map = !self.show_sprites_in_map;
            }
            Msg::HoveredTile(hovered_tile) => {
                self.hovered_tile = hovered_tile;
            }
        }
    }

    pub(crate) fn subscriptions(event: &Event) -> Option<Msg> {
        match event {
            Event::Input(InputEvent::Keyboard(event)) => {
                let KeyboardEvent { key, state } = event;

                match (key, state) {
                    (Key::C, KeyState::Down) => Some(Msg::SwitchMapMode),
                    (Key::Space, key_state) => Some(Msg::SetDragging(*key_state == KeyState::Down)),
                    _ => None,
                }
            }
            &Event::Input(InputEvent::Mouse(MouseEvent::Move { x, y })) => {
                Some(Msg::MouseMove(vec2(x, y)))
            }
            _ => None,
        }
    }

    pub(crate) fn view<'a, 'b, Msg: Copy + Debug + 'a>(
        &'a mut self,
        map: &'b Map,
        x: i32,
        y: i32,
        on_tile_click: &impl Fn(usize, usize) -> Msg,
        on_map_editor_msg: &impl Fn(self::Msg) -> Msg,
    ) -> Element<'a, Msg> {
        let show_sprites_in_map = self.show_sprites_in_map;
        let camera = self.camera;

        dbg!(camera);

        const W: i32 = Sprite::WIDTH as i32;
        let num_to_fill_on_the_left = (camera.x + W - 1) / W;
        let num_to_fill_on_the_left = num_to_fill_on_the_left.max(0);

        dbg!(num_to_fill_on_the_left);

        let highlighted_tile_position =
            tile_position(self.camera, self.hovered_tile.0, self.hovered_tile.1) + vec2(x, y);

        /// How many tiles is the map displayable area in height?
        /// It's actually 8.5 tiles, but we need to fill 9.
        const MAP_H_TILES: i32 = 9;

        let filler_left = (0..num_to_fill_on_the_left)
            .cartesian_product(0..MAP_H_TILES + 1)
            .map(|(index_x, index_y)| {
                let x = camera.x - (index_x + 1) * W;
                let y = camera.y % W + y + index_y * 8;

                DrawFn::<'_, Msg>::new(move |pico8| {
                    // TODO: Reset palette
                    pico8.palt(None);
                    pico8.spr(Self::FILLER_SPR, x, y);
                    pico8.palt(Some(0));
                })
                .into()
            });

        let buttons_iter = self.buttons.iter_mut().chunks(16);
        let map = buttons_iter
            .into_iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.into_iter().enumerate().map(move |(col_index, state)| {
                    let sprite = map.mget(col_index as i32, row_index as i32);

                    let Vec2i { x, y } = tile_position(camera, col_index, row_index) + vec2(x, y);
                    Button::new(
                        x,
                        y,
                        8,
                        8,
                        Some(on_tile_click(col_index, row_index)),
                        state,
                        DrawFn::new(move |draw| {
                            draw.palt(None);
                            if show_sprites_in_map {
                                draw.spr(sprite.into(), 0, 0);
                            } else {
                                draw.print(&format!("{sprite:0>2X}"), 0, 1, 7);
                            }
                        }),
                    )
                    .event_on_press()
                    .on_hover(on_map_editor_msg(self::Msg::HoveredTile((
                        col_index, row_index,
                    ))))
                    .into()
                })
            });

        Tree::with_children(filler_left.chain(map).collect())
            .push(highlight_hovered(highlighted_tile_position))
            .into()
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Msg {
    SwitchMapMode,
    HoveredTile((usize, usize)),
    MouseMove(Vec2i),
    SetDragging(bool),
}

fn highlight_hovered<'a, Msg: Copy + Debug + 'a>(tile_position: Vec2i) -> Element<'a, Msg> {
    DrawFn::new(move |draw| {
        draw.rect(
            tile_position.x,
            tile_position.y,
            tile_position.x + 7,
            tile_position.y + 7,
            7,
        )
    })
    .into()
}

fn tile_position(camera: Vec2i, col_index: usize, row_index: usize) -> Vec2i {
    camera + vec2(col_index as i32 * 8, row_index as i32 * 8)
}
