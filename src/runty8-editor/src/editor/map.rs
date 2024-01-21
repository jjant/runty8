use crate::ui::button::{self, Button};
use crate::ui::{DrawFn, Element, Tree};
use crate::util::vec2::{vec2, Vec2i};
use itertools::Itertools;
use runty8_core::colors::{
    BLACK, BLUE, BROWN, DARK_BLUE, DARK_GREEN, DARK_GREY, DARK_PURPLE, GREEN, LAVENDER, LIGHT_GREY,
    LIGHT_PEACH, ORANGE, PINK, RED, WHITE, YELLOW,
};
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
    drag_offset: Vec2i,
}

impl Editor {
    /// How many tiles is the map displayable area in height?
    /// It's actually 8.5 tiles, but we need to fill 9.
    const MAP_H_TILES: i32 = 9;

    const FILLER_SPR: usize = 1;

    pub(crate) fn new() -> Self {
        Self {
            buttons: vec![button::State::new(); 144],
            show_sprites_in_map: true,
            hovered_tile: (0, 0),
            mouse_position: vec2(64, 64),
            camera: Vec2i::zero(),
            dragging: false,
            drag_offset: vec2(0, 0),
        }
    }

    pub(crate) fn update(&mut self, msg: Msg) {
        match msg {
            Msg::MouseMove(mouse_position) => {
                let raw_delta = self.mouse_position - mouse_position;

                // println!("[Map Editor] Mouse delta: {delta:?}");

                self.mouse_position = mouse_position;
                if self.dragging {
                    // self.camera = self.camera - delta;
                    self.drag_offset = self.drag_offset + raw_delta;
                    //x
                    let delta = {
                        let tiles_to_move = self.drag_offset / 8;
                        let delta = 8 * tiles_to_move;
                        self.drag_offset = self.drag_offset - delta;

                        delta
                    };
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
        let camera = self.camera;

        dbg!(camera);

        const W: i32 = Sprite::WIDTH as i32;
        let num_to_fill_on_the_left = (camera.x + W - 1) / W;
        let num_to_fill_on_the_left = num_to_fill_on_the_left.max(0);

        dbg!(num_to_fill_on_the_left);

        let hovered_tile_highlight = if !self.dragging {
            self.highlight_hovered(x, y)
        }
        // Hide highlight while moving the map around
        else {
            Tree::new().into()
        };

        let top_padding = self.top_padding(y);
        let filler = self.filler(x, y);

        Tree::new()
            .push(filler)
            .push(self.actual_map(x, y, map, on_tile_click, on_map_editor_msg))
            .push(hovered_tile_highlight)
            .push(top_padding)
            .into()
    }

    fn filler<'a, Msg: Copy + Debug + 'a>(&self, base_x: i32, base_y: i32) -> Element<'a, Msg> {
        DrawFn::new(move |pico8| {
            pico8.palt(None);
            // TODO: Use a common constant for this
            // This is equal to 128(pixels)/8(pixels/tile)
            let width_tiles = 16;
            for column in 0..width_tiles {
                for row in 0..Self::MAP_H_TILES {
                    let x = column * 8 + base_x;

                    let top_padding = 1;
                    let y = row * 8 + base_y + top_padding;
                    pico8.spr(Self::FILLER_SPR, x, y);
                }
            }
            pico8.palt(Some(0));
        })
        .into()
    }

    fn actual_map<'a, 'b, Msg: Copy + Debug + 'a>(
        &'a mut self,
        x: i32,
        y: i32,
        map: &'b Map,
        on_tile_click: &impl Fn(usize, usize) -> Msg,
        on_map_editor_msg: &impl Fn(self::Msg) -> Msg,
    ) -> Vec<Element<'a, Msg>> {
        let buttons_iter = self.buttons.iter_mut().chunks(16);
        let camera = self.camera;
        let show_sprites_in_map = self.show_sprites_in_map;

        buttons_iter
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
            })
            .collect()
    }

    fn top_padding<'a, Msg: Copy + Debug + 'a>(&self, y: i32) -> Element<'a, Msg> {
        DrawFn::new(move |pico8| {
            // Note: The "darkening" function for the 4-tile "separators"
            // (shown when holding down the space bar) is different.
            fn darken_pixel(pixel: u8) -> u8 {
                match pixel {
                    BLACK => BLACK,
                    DARK_BLUE => BLACK,
                    DARK_PURPLE => DARK_GREY,
                    DARK_GREEN => DARK_BLUE,
                    BROWN => DARK_PURPLE,
                    DARK_GREY => DARK_BLUE,
                    LIGHT_GREY => LAVENDER,
                    WHITE => LIGHT_GREY,
                    RED => DARK_PURPLE,
                    ORANGE => BROWN,
                    YELLOW => ORANGE,
                    GREEN => DARK_GREEN,
                    BLUE => LAVENDER,
                    LAVENDER => DARK_GREY,
                    PINK => LAVENDER,
                    LIGHT_PEACH => LIGHT_GREY,
                    16_u8..=u8::MAX => 0,
                }
            }
            pico8.palt(None);
            for x in 0..128 {
                let color = pico8.pget(x, y);
                pico8.pset(x, y, darken_pixel(color));
            }
            pico8.palt(Some(0));
        })
        .into()
    }

    fn highlight_hovered<'a, Msg: Copy + Debug + 'a>(&self, x: i32, y: i32) -> Element<'a, Msg> {
        let tile_position =
            tile_position(self.camera, self.hovered_tile.0, self.hovered_tile.1) + vec2(x, y);

        DrawFn::new(move |pico8| {
            pico8.rect(
                tile_position.x - 1,
                tile_position.y - 1,
                tile_position.x + 8,
                tile_position.y + 8,
                7,
            )
        })
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

fn tile_position(camera: Vec2i, col_index: usize, row_index: usize) -> Vec2i {
    camera + vec2(col_index as i32 * 8, row_index as i32 * 8)
}
