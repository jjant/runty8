use crate::ui::button::{self, Button};
use crate::ui::{DrawFn, Element, Tree};
use crate::Map;
use crate::{Event, Key, KeyState, KeyboardEvent};
use itertools::Itertools;
use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct Editor {
    buttons: Vec<button::State>,
    show_sprites_in_map: bool,
    hovered_tile: (usize, usize),
}

impl Editor {
    pub(crate) fn new() -> Self {
        Self {
            buttons: vec![button::State::new(); 144],
            show_sprites_in_map: true,
            hovered_tile: (0, 0),
        }
    }

    pub(crate) fn update(&mut self, msg: Msg) {
        match msg {
            Msg::SwitchMapMode => {
                self.show_sprites_in_map = !self.show_sprites_in_map;
            }
            Msg::HoveredTile(hovered_tile) => {
                self.hovered_tile = hovered_tile;
            }
        }
    }

    pub(crate) fn subscriptions(event: &Event) -> Option<Msg> {
        if let Event::Keyboard(event) = event {
            match event {
                KeyboardEvent {
                    key: Key::C,
                    state: KeyState::Down,
                } => Some(Msg::SwitchMapMode),
                _ => None,
            }
        } else {
            None
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

        let v: Vec<Element<'_, Msg>> = self
            .buttons
            .iter_mut()
            // .zip(map)
            .chunks(16)
            .into_iter()
            .take(9) // 9 rows
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.into_iter().enumerate().map(move |(col_index, state)| {
                    let sprite = map.mget(col_index as i32, row_index as i32);
                    let x = x as usize + col_index * 8;
                    let y = y as usize + row_index * 8;

                    Button::new(
                        x as i32,
                        y as i32,
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
            .push(highlight_hovered(self.hovered_tile, x, y))
            .into()
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Msg {
    SwitchMapMode,
    HoveredTile((usize, usize)),
}

fn highlight_hovered<'a, Msg: Copy + Debug + 'a>(
    hovered_tile: (usize, usize),
    x: i32,
    y: i32,
) -> Element<'a, Msg> {
    let (col_index, row_index) = hovered_tile;
    let x = x as usize + col_index * 8;
    let y = y as usize + row_index * 8;

    DrawFn::new(move |draw| draw.rect(x as i32, y as i32, (x + 7) as i32, (y + 7) as i32, 7)).into()
}