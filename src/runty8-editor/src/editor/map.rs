use crate::ui::button::{self, Button};
use crate::ui::{DrawFn, Element, Tree};
use crate::util::vec2::{vec2, Vec2i};
use crate::Map;
use itertools::Itertools;
use runty8_core::{Event, Key, KeyState, KeyboardEvent, MouseEvent};
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

                println!("[Map Editor] Mouse delta: {:?}", delta);

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
            Event::Keyboard(event) => {
                let KeyboardEvent { key, state } = event;

                match (key, state) {
                    (Key::C, KeyState::Down) => Some(Msg::SwitchMapMode),
                    (Key::Space, key_state) => Some(Msg::SetDragging(*key_state == KeyState::Down)),
                    _ => None,
                }
            }
            &Event::Mouse(MouseEvent::Move { x, y }) => Some(Msg::MouseMove(vec2(x, y))),
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

        let highlighted_tile_position =
            tile_position(self.camera, self.hovered_tile.0, self.hovered_tile.1) + vec2(x, y);

        let v: Vec<Element<'_, Msg>> = self
            .buttons
            .iter_mut()
            .chunks(16)
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
                                draw.print(&format!("{:0>2X}", sprite), 0, 1, 7);
                            }
                        }),
                    )
                    .event_on_press()
                    .on_hover(on_map_editor_msg(self::Msg::HoveredTile((
                        col_index, row_index,
                    ))))
                    .into()
                })
            })
            .collect();

        Tree::with_children(v)
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
