use std::path::Path;

use itertools::Itertools;
use runty8::runtime::draw_context::DrawData;
use runty8::runtime::sprite_sheet::SpriteSheet;
use runty8::runtime::state::{Flags, State};
use runty8::runtime::{cmd::Cmd, map::Map};
use runty8::ui::button::{self, Button};
use runty8::ui::{
    cursor::{self, Cursor},
    text::Text,
    ElmApp2,
};
use runty8::ui::{DrawFn, Element, Sub, Tree};

fn create_directory() -> &'static str {
    let buf = Path::new(file!()).with_extension("");
    let dir_name = buf.to_str().unwrap();

    if let Err(e) = std::fs::create_dir(dir_name) {
        println!("Couldn't create directory, error: {:?}", e);
    };

    Box::leak(Box::from(dir_name))
}

fn create_sprite_flags(assets_path: &str) -> Flags {
    if let Ok(content) = std::fs::read_to_string(&format!(
        "{}{}{}",
        assets_path,
        std::path::MAIN_SEPARATOR,
        Flags::file_name()
    )) {
        Flags::deserialize(&content).unwrap()
    } else {
        Flags::new()
    }
}

fn create_sprite_sheet(assets_path: &str) -> SpriteSheet {
    if let Ok(content) = std::fs::read_to_string(&format!(
        "{}{}{}",
        assets_path,
        std::path::MAIN_SEPARATOR,
        SpriteSheet::file_name()
    )) {
        SpriteSheet::deserialize(&content).unwrap()
    } else {
        SpriteSheet::new()
    }
}

fn main() {
    let assets_path = create_directory();

    let map: &'static Map = Box::leak(Box::new(Map::new()));
    let sprite_flags: &'static Flags = Box::leak(Box::new(create_sprite_flags(assets_path)));
    let sprite_sheet = create_sprite_sheet(assets_path);

    let state = State::new(assets_path, sprite_sheet, sprite_flags, map);
    let draw_data = DrawData::new();

    runty8::screen::run_app::<MyApp>((map, sprite_flags), state, draw_data);
}

#[derive(Debug)]
struct MyApp<'a> {
    map: &'a Map,
    flags: &'a Flags,
    cursor: cursor::State,
    tab: Tab,
    selected_color: u8,
    selected_sprite: usize,
    selected_sprite_page: usize,
    sprite_button_state: button::State,
    map_button_state: button::State,
    tab_buttons: [button::State; 4],
    color_selector_state: Vec<button::State>,
    flag_buttons: Vec<button::State>,
    sprite_buttons: Vec<button::State>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tab {
    SpriteEditor,
    MapEditor,
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    SpriteTabClicked,
    MapButtonClicked,
    ColorSelected(usize),
    SpritePageSelected(usize),
    SpriteButtonClicked(usize),
    FlagToggled(usize),
}

impl<'a> ElmApp2 for MyApp<'a> {
    type Msg = Msg;
    type Flags = (&'a Map, &'a Flags);

    fn init((map, flags): Self::Flags) -> (Self, Cmd<Msg>) {
        let selected_sprite = 0;

        (
            Self {
                map,
                flags,
                cursor: cursor::State::new(),
                sprite_button_state: button::State::new(),
                map_button_state: button::State::new(),
                tab: Tab::SpriteEditor,
                selected_color: 0,
                selected_sprite,
                selected_sprite_page: 0,
                tab_buttons: [
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                ],
                color_selector_state: vec![button::State::new(); 16],
                flag_buttons: vec![button::State::new(); 8],
                sprite_buttons: vec![button::State::new(); 64],
            },
            Cmd::none(),
        )
    }

    fn update(&mut self, msg: &Self::Msg) -> Cmd<Msg> {
        match msg {
            Msg::SpriteTabClicked => {
                self.tab = Tab::SpriteEditor;
                println!("Sprite button clicked");
            }
            Msg::MapButtonClicked => {
                self.tab = Tab::MapEditor;
                println!("Map button clicked");
            }
            Msg::ColorSelected(selected_color) => {
                self.selected_color = *selected_color as u8;
            }
            Msg::SpritePageSelected(selected_sprite_page) => {
                self.selected_sprite_page = *selected_sprite_page;
            }
            Msg::SpriteButtonClicked(selected_sprite) => {
                self.selected_sprite = *selected_sprite;
            }
            Msg::FlagToggled(flag_index) => {
                let flag_index = *flag_index;

                let flag_value = self.flags.fget_n(self.selected_sprite, flag_index as u8);
                self.flags
                    .fset(self.selected_sprite, flag_index, !flag_value);
            }
        }

        Cmd::none()
    }

    fn view(&mut self) -> Element<'_, Self::Msg> {
        const BACKGROUND: u8 = 5;

        let bottom_bar_text = "THIS IS THE BOT BAR".to_owned();

        Tree::new()
            .push(DrawFn::new(|draw| {
                draw.rectfill(0, 0, 127, 127, BACKGROUND)
            }))
            .push(top_bar(
                &mut self.sprite_button_state,
                &mut self.map_button_state,
                self.tab,
            ))
            .push(match self.tab {
                Tab::SpriteEditor => sprite_editor_view(
                    self.selected_color,
                    &mut self.color_selector_state,
                    self.flags.get(self.selected_sprite).unwrap(),
                    &mut self.flag_buttons,
                ),
                Tab::MapEditor => map_view(self.map, 0, 8),
            })
            .push(tools_row(
                76,
                self.selected_sprite,
                self.selected_sprite_page,
                &mut self.tab_buttons,
            ))
            .push(sprite_view(
                self.selected_sprite,
                self.selected_sprite_page,
                &mut self.sprite_buttons,
                87,
            ))
            .push(bottom_bar(bottom_bar_text))
            .push(Cursor::new(&mut self.cursor))
            .into()
    }

    fn subscriptions(&self) -> Sub<Self::Msg> {
        Sub::NoSub
    }
}

fn top_bar<'a>(
    sprite_button_state: &'a mut button::State,
    map_button_state: &'a mut button::State,
    tab: Tab,
) -> Element<'a, Msg> {
    Tree::new()
        .push(DrawFn::new(|draw| {
            draw.rectfill(0, 0, 127, 7, 8);
        }))
        .push(sprite_editor_button(sprite_button_state, tab))
        .push(map_editor_button(map_button_state, tab))
        .into()
}

#[allow(clippy::too_many_arguments)]
fn sprite_editor_view<'a>(
    selected_color: u8,
    color_selector_state: &'a mut [button::State],
    selected_sprite_flags: u8,
    flag_buttons: &'a mut [button::State],
) -> Element<'a, Msg> {
    Tree::new()
        .push(color_selector(
            79,
            10,
            10,
            selected_color,
            color_selector_state,
            Msg::ColorSelected,
        ))
        .push(flags(selected_sprite_flags, 78, 70, flag_buttons))
        .into()
}

fn map_view(map: &Map, x: i32, y: i32) -> Element<'_, Msg> {
    let v: Vec<Element<'_, Msg>> = map
        .iter()
        .chunks(16)
        .into_iter()
        .take(9)
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.into_iter().enumerate().map(move |(col_index, sprite)| {
                DrawFn::new(move |draw| {
                    let x = x as usize + col_index * 8;
                    let y = y as usize + row_index * 8;

                    draw.spr(sprite.into(), x as i32, y as i32);
                })
                .into()
            })
        })
        .collect();

    Tree::with_children(v).into()
}

fn sprite_editor_button(state: &mut button::State, tab: Tab) -> Element<'_, Msg> {
    let selected = tab == Tab::SpriteEditor;

    Button::new(
        110,
        0,
        8,
        8,
        Some(Msg::SpriteTabClicked),
        state,
        DrawFn::new(move |draw| {
            let color = if selected { 2 } else { 15 };

            draw.pal(15, color);
            draw.spr(63, 0, 0);
            draw.pal(15, 15);
        }),
    )
    .into()
}

fn map_editor_button(state: &mut button::State, tab: Tab) -> Element<'_, Msg> {
    let selected = tab == Tab::MapEditor;

    Button::new(
        118,
        0,
        8,
        8,
        Some(Msg::MapButtonClicked),
        state,
        DrawFn::new(move |draw| {
            let color = if selected { 2 } else { 15 };

            draw.pal(15, color);
            draw.spr(62, 0, 0);
            draw.pal(15, 15);
        }),
    )
    .into()
}

fn color_selector<'a>(
    start_x: i32,
    start_y: i32,
    tile_size: i32,
    selected_color: u8,
    states: &'a mut [button::State],
    on_press: impl (Fn(usize) -> Msg) + Copy + 'static,
) -> Element<'_, Msg> {
    let mut v = Vec::with_capacity(16);

    let coordinates = move |index| {
        let i = index % 4;
        let j = index / 4;
        let x = start_x + 1 + i as i32 * tile_size;
        let y = start_y + 1 + j as i32 * tile_size;

        (x, y)
    };

    for (index, state) in states.iter_mut().enumerate() {
        let (x, y) = coordinates(index);

        let button: Element<'_, Msg> = Button::new(
            x,
            y,
            tile_size,
            tile_size,
            Some(on_press(index)),
            state,
            DrawFn::new(move |draw| {
                draw.palt(None);
                draw.rectfill(0, 0, tile_size - 1, tile_size - 1, index as u8);
                draw.palt(Some(0));
            }),
        )
        .into();
        v.push(button);
    }

    // Draw border
    v.push(
        DrawFn::new(move |draw| {
            draw.palt(None);
            draw.rect(
                start_x,
                start_y,
                start_x + 4 * tile_size + 1,
                start_y + 4 * tile_size + 1,
                0,
            );
            draw.palt(Some(0));
        })
        .into(),
    );

    // Draw highlight
    v.push(
        DrawFn::new(move |draw| {
            let (x, y) = coordinates(selected_color as usize);

            draw.palt(None);
            draw.rect(x, y, x + tile_size - 1, y + tile_size - 1, 0);
            draw.rect(x - 1, y - 1, x + tile_size, y + tile_size, 7);
            draw.palt(Some(0));
        })
        .into(),
    );

    Tree::with_children(v).into()
}

fn tools_row(
    y: i32,
    sprite: usize,
    selected_tab: usize,
    tab_buttons: &mut [button::State],
) -> Element<'_, Msg> {
    let mut children = vec![DrawFn::new(move |draw| {
        const HEIGHT: i32 = 11;
        draw.rectfill(0, y, 127, y + HEIGHT - 1, 5)
    })
    .into()];

    for (sprite_tab, tab_button_state) in tab_buttons.iter_mut().enumerate() {
        let base_sprite = if selected_tab == sprite_tab { 33 } else { 17 };

        let x = 96 + sprite_tab as i32 * 8;

        children.push(
            Button::new(
                x,
                y + 3,
                8,
                8,
                Some(Msg::SpritePageSelected(sprite_tab)),
                tab_button_state,
                DrawFn::new(move |draw| {
                    draw.palt(Some(0));
                    draw.spr(base_sprite + sprite_tab, 0, 0);
                }),
            )
            .into(),
        );
    }

    const X: i32 = 70;
    let sprite_preview = spr(sprite, X, y + 2);
    children.push(sprite_preview.into());

    let spr_str = format!("{:0>3}", sprite);
    let sprite_number = DrawFn::new(move |draw| {
        let y = y + 2;
        draw.rectfill(X + 9, y + 1, X + 9 + 13 - 1, y + 7, 6);
        draw.print(&spr_str, X + 10, y + 2, 13);
    })
    .into();
    children.push(sprite_number);

    Tree::with_children(children).into()
}

/// The 4 rows of sprites at the bottom of the sprite editor
fn sprite_view(
    selected_sprite: usize,
    selected_tab: usize,
    sprite_buttons: &mut [button::State],
    y: i32,
) -> Element<'_, Msg> {
    let mut children = vec![DrawFn::new(move |draw| {
        draw.palt(None);
        draw.rectfill(0, y, 127, y + 32 + 1, 0);
    })
    .into()];

    let sprite_position = |sprite| {
        let index = sprite % 64;
        let row = (index / 16) as i32;
        let col = (index % 16) as i32;

        (col * 8, y + 1 + row * 8)
    };

    for (index, sprite_state) in sprite_buttons.iter_mut().enumerate() {
        let sprite = index + selected_tab * 64;

        let (x, y) = sprite_position(sprite);
        children.push(
            Button::new(
                x,
                y,
                8,
                8,
                Some(Msg::SpriteButtonClicked(sprite)),
                sprite_state,
                DrawFn::new(move |draw| {
                    draw.palt(None);
                    draw.spr(sprite, 0, 0);
                }),
            )
            .into(),
        );
    }

    // Draw selected sprite highlight
    {
        // TODO: Fix (wrong highlight when switching pages)
        let (x, y) = sprite_position(selected_sprite);
        children.push(
            DrawFn::new(move |draw| {
                draw.rect(x - 1, y - 1, x + 8, y + 8, 7);
            })
            .into(),
        )
    }

    Tree::with_children(children).into()
}

fn bottom_bar(text: String) -> Element<'static, Msg> {
    const X: i32 = 0;
    const Y: i32 = 121;
    const BAR_WIDTH: i32 = 128;
    const BAR_HEIGHT: i32 = 7;

    Tree::new()
        .push(DrawFn::new(|draw| {
            draw.rectfill(X, Y, X + BAR_WIDTH - 1, Y + BAR_HEIGHT - 1, 8)
        }))
        .push(Text::new(text, X + 1, Y + 1, 2))
        .into()
}

// TODO:
// - Change color of highlight
// - Don't show button underneath
// - Optimize? (no Tree::new with draw commands)
fn flags(
    selected_sprite_flags: u8,
    x: i32,
    y: i32,
    flag_buttons: &mut [button::State],
) -> Element<'_, Msg> {
    const SPR_SIZE: i32 = 5;
    const FLAG_COLORS: [u8; 8] = [8, 9, 10, 11, 12, 13, 14, 15];

    let children = flag_buttons
        .iter_mut()
        .enumerate()
        .map(|(index, button)| {
            let x = x + (SPR_SIZE + 1) * index as i32;
            let flag_on = selected_sprite_flags & (1 << index) != 0;
            let color = if flag_on { FLAG_COLORS[index] } else { 1 };

            let button_content: Element<'_, Msg> = Tree::new()
                .push(palt(Some(7)))
                .push(pal(1, color))
                .push(spr(58, 0, 0))
                .push(pal(1, 1))
                .push(palt(Some(0)))
                .into();

            let button = Button::new(
                x,
                y,
                5,
                5,
                Some(Msg::FlagToggled(index)),
                button,
                button_content,
            );

            button.into()
        })
        .collect();

    Tree::with_children(children).into()
}

fn palt(transparent_color: Option<u8>) -> impl Into<Element<'static, Msg>> {
    DrawFn::new(move |draw| draw.palt(transparent_color))
}

fn pal(c0: u8, c1: u8) -> impl Into<Element<'static, Msg>> {
    DrawFn::new(move |draw| draw.pal(c0, c1))
}

fn spr(sprite: usize, x: i32, y: i32) -> impl Into<Element<'static, Msg>> {
    DrawFn::new(move |draw| draw.spr(sprite, x, y))
}
