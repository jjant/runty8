use crate::pico8::Pico8EditorExt;
use crate::ui::button::{self, Button};
use crate::ui::{DrawFn, Element, Tree};
use crate::util::vec2::{vec2, Vec2i};
use itertools::Itertools;
use runty8_core::colors::{
    self, BLACK, BLUE, BROWN, DARK_BLUE, DARK_GREEN, DARK_GREY, DARK_PURPLE, GREEN, LAVENDER,
    LIGHT_GREY, LIGHT_PEACH, ORANGE, PINK, RED, WHITE, YELLOW,
};
use runty8_core::{Event, InputEvent, Key, KeyState, KeyboardEvent, MouseEvent, Sprite};
use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct Editor {
    buttons: Vec<button::State>,
    show_sprites_in_map: bool,
    hovered_tile: (usize, usize),
    mouse_position: Vec2i,
    /// Offset in x,y in number of tiles.
    /// ```
    /// camera: vec2(5, 1)
    /// ```
    /// Means that tile (5, 1) is rendered in the top-left corner of the map.
    camera: Vec2i,
    // TODO: Use a proper enum
    dragging: bool,
    drag_offset: Vec2i,
}

impl Editor {
    /// How many tiles is the map displayable area in height?
    /// It's actually 8.5 tiles, but we need to fill 9.
    // const MAP_H_TILES: i32 = 9;

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

                    let delta_tiles = {
                        let delta_tiles = self.drag_offset / 8;
                        self.drag_offset = self.drag_offset - 8 * delta_tiles;

                        delta_tiles
                    };
                    self.camera = self.camera + delta_tiles;
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
        // let filler = self.filler(x, y);
        let map_boundary_border = self.map_boundary_border(x, y);
        let drag_grid = self.drag_grid();

        Tree::new()
            // .push(filler)
            .push(self.actual_map(x, y, on_tile_click, on_map_editor_msg))
            .push(drag_grid)
            .push(hovered_tile_highlight)
            .push(map_boundary_border)
            .push(top_padding)
            .into()
    }

    fn actual_map<'a, 'b, Msg: Copy + Debug + 'a>(
        &'a mut self,
        x: i32,
        y: i32,
        on_tile_click: &impl Fn(usize, usize) -> Msg,
        on_map_editor_msg: &impl Fn(self::Msg) -> Msg,
    ) -> Vec<Element<'a, Msg>> {
        let camera = self.camera;
        let show_sprites_in_map = self.show_sprites_in_map;

        let buttons_iter = self.buttons.iter_mut().chunks(16);
        buttons_iter
            .into_iter()
            .enumerate()
            .flat_map(move |(row, button_row)| {
                button_row.enumerate().map(move |(col, state)| {
                    let Vec2i { x, y } = tile_position(col, row) + vec2(x, y);

                    let Vec2i {
                        x: map_tile_x,
                        y: map_tile_y,
                    } = vec2(col, row).try_convert::<i32>().unwrap() + camera;

                    let inside_map = inside_map(map_tile_x, map_tile_y);
                    let on_tile_click = if inside_map {
                        // This conversion is fine because we checked the range in `inside_map`.
                        Some(on_tile_click(map_tile_x as usize, map_tile_y as usize))
                    } else {
                        None
                    };
                    Button::new(
                        x,
                        y,
                        // TODO: Use constants
                        8,
                        8,
                        on_tile_click,
                        state,
                        DrawFn::new(move |pico8| {
                            pico8.palt(None);
                            if inside_map {
                                pico8.map(map_tile_x, map_tile_y, 0, 0, 1, 1, 0);
                                if !show_sprites_in_map {
                                    let sprite = pico8.mget(map_tile_x, map_tile_y);
                                    pico8.print(&format!("{sprite:0>2X}"), 0, 1, 7);
                                }
                            } else {
                                pico8.editor_spr(Self::FILLER_SPR, 0, 0);
                            }
                            pico8.palt(Some(0));
                        }),
                    )
                    .event_on_press()
                    .on_hover(on_map_editor_msg(self::Msg::HoveredTile((col, row))))
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
        let tile_position = tile_position(self.hovered_tile.0, self.hovered_tile.1) + vec2(x, y);

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

    fn map_boundary_border<'a, Msg: Copy + Debug + 'a>(&self, x: i32, y: i32) -> Element<'a, Msg> {
        let camera = self.camera;

        DrawFn::new(move |pico8| {
            let start_relative = vec2(-2, -2);
            let border_size_in_pixels = vec2(
                runty8_core::Map::WIDTH_PIXELS + 1,
                runty8_core::Map::HEIGHT_PIXELS + 1,
            )
            .try_convert()
            .unwrap();

            let start_global = start_relative + vec2(x, y) - 8 * camera;
            let end_global = start_global + border_size_in_pixels;

            // Grey outer border
            pico8.rect(
                start_global.x,
                start_global.y,
                end_global.x + 2,
                end_global.y + 2,
                LIGHT_GREY,
            );
            // Black inner border
            pico8.rect(
                start_global.x + 1,
                start_global.y + 1,
                end_global.x + 1,
                end_global.y + 1,
                BLACK,
            );
        })
        .into()
    }

    // TODO: Rename.
    fn drag_grid<'a, Msg: Copy + Debug + 'a>(&self) -> Element<'a, Msg> {
        const CROSS_SPR: usize = 31;
        let camera_in_tiles = self.camera * 8;
        let base_x = 0;
        let base_y = 8;

        let lines = (0..9)
            .into_iter()
            .flat_map(|col| {
                [
                    DrawFn::new(move |pico8| {
                        let x = base_x + col * 16 - camera_in_tiles.x;
                        let y = base_y - camera_in_tiles.y;
                        pico8.line(x, y, x, y + 127, colors::DARK_GREY);
                    }),
                    DrawFn::new(move |pico8| {
                        let x = base_x - camera_in_tiles.x;
                        let y = base_y + col * 16 - camera_in_tiles.y;
                        pico8.line(x, y, x + 127, y, colors::DARK_GREY);
                    }),
                ]
            })
            .map(|draw_fn| draw_fn.into())
            .collect::<Vec<_>>();

        let crosses = (0..3)
            .flat_map(|col| {
                (0..3).map(move |row| {
                    DrawFn::new(move |pico8| {
                        let x = base_x + col * 4 * 16 - camera_in_tiles.x;
                        let y = base_y + row * 4 * 16 - camera_in_tiles.y;

                        pico8.editor_spr(CROSS_SPR, x - 2, y - 2);
                    })
                    .into()
                })
            })
            .collect();

        Tree::with_children(lines).append(crosses).into()
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Msg {
    SwitchMapMode,
    HoveredTile((usize, usize)),
    MouseMove(Vec2i),
    SetDragging(bool),
}

fn tile_position(col_index: usize, row_index: usize) -> Vec2i {
    vec2(col_index as i32 * 8, row_index as i32 * 8)
}

/// Returns whether a tile coordinate is inside the map.
fn inside_map(tile_x: i32, tile_y: i32) -> bool {
    // TODO: Use constants from [`runty8_core::Map`].
    let inside_x = (0..=127).contains(&tile_x);
    let inside_y = (0..=63).contains(&tile_y);

    inside_x && inside_y
}
