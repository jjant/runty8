use itertools::Itertools;
use runty8::runtime::draw_context::DrawData;
use runty8::runtime::state::{Flags, State};
use runty8::runtime::{cmd::Cmd, map::Map};
use runty8::ui::button::{self, Button};
use runty8::ui::{
    cursor::{self, Cursor},
    ElmApp2,
};
use runty8::ui::{DrawFn, Element, Sub, Tree, Widget};

fn main() {
    let map: &'static Map = Box::leak(Box::new(Map::new()));
    let sprite_flags: &'static Flags = Box::leak(Box::new(Flags::new()));

    let state = State::new(map, sprite_flags);
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
    current_flags: u8,
    sprite_button_state: button::State,
    map_button_state: button::State,
    tab_buttons: [button::State; 4],
    color_selector_state: [button::State; 16],
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
    GotFlags(u8),
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
                color_selector_state: [
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                    button::State::new(),
                ],
                flag_buttons: vec![button::State::new(); 8],
                sprite_buttons: vec![button::State::new(); 64],
                current_flags: 0,
            },
            Cmd::get_flags(selected_sprite as u8).map(Msg::GotFlags),
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

                return Cmd::get_flags(self.selected_sprite as u8).map(Msg::GotFlags);
            }
            Msg::FlagToggled(flag) => {
                let sprite = self.selected_sprite as u8;

                // self.flags
                // return Cmd::toggle_flag(sprite, *flag as u8)
                //     .and_then(move |_| Cmd::get_flags(sprite))
                //     .map(Msg::GotFlags);
                return Cmd::none();
            }
            Msg::GotFlags(flags) => {
                self.current_flags = *flags;
            }
        }

        Cmd::none()
    }

    fn view(&mut self) -> Element<'_, Self::Msg> {
        let top_bar = DrawFn::new(|draw| {
            draw.rectfill(0, 0, 127, 7, 8);
        });

        const BACKGROUND: u8 = 5;

        let view: Vec<Element<'_, Msg>> = vec![
            DrawFn::new(|draw| draw.rectfill(0, 0, 127, 127, BACKGROUND)),
            top_bar,
            match self.tab {
                Tab::SpriteEditor => sprite_editor_view(
                    self.tab,
                    self.selected_sprite,
                    self.selected_sprite_page,
                    self.selected_color,
                    &mut self.sprite_button_state,
                    &mut self.map_button_state,
                    &mut self.color_selector_state,
                    &mut self.tab_buttons,
                    &mut self.sprite_buttons,
                    self.flags.get(self.selected_sprite).unwrap(),
                    &mut self.flag_buttons,
                ),
                Tab::MapEditor => self.map_view(0, 32),
            },
            bottom_bar(),
            Cursor::new(&mut self.cursor),
        ];

        view.into()
    }

    fn subscriptions(&self) -> Sub<Self::Msg> {
        Sub::NoSub
    }
}

#[allow(clippy::too_many_arguments)]
fn sprite_editor_view<'a>(
    tab: Tab,
    selected_sprite: usize,
    selected_sprite_page: usize,
    selected_color: u8,
    sprite_button_state: &'a mut button::State,
    map_button_state: &'a mut button::State,
    color_selector_state: &'a mut [button::State],
    tab_buttons: &'a mut [button::State],
    sprite_buttons: &'a mut [button::State],
    current_flags: u8,
    flag_buttons: &'a mut [button::State],
) -> Element<'a, Msg> {
    let v: Vec<Element<'_, Msg>> = vec![
        sprite_editor_button(sprite_button_state, tab),
        map_editor_button(map_button_state, tab),
        color_selector(
            79,
            10,
            10,
            selected_color,
            color_selector_state,
            Msg::ColorSelected,
        ),
        sprite_view(
            selected_sprite,
            selected_sprite_page,
            sprite_buttons,
            tab_buttons,
            87,
        ),
        sprite_preview(selected_sprite, 71, 78),
        flags(current_flags, 78, 70, flag_buttons),
    ];

    v.into()
}
impl<'map> MyApp<'map> {
    fn map_view(&mut self, x: i32, y: i32) -> Element<'static, Msg> {
        dbg!(self.map.iter().count());

        let v: Vec<Element<'_, Msg>> = self
            .map
            .iter()
            .chunks(16)
            .into_iter()
            .take(3)
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.into_iter().enumerate().map(move |(col_index, sprite)| {
                    let f: Element<'static, Msg> = DrawFn::new(move |draw| {
                        let x = x as usize + col_index * 8;
                        let y = y as usize + row_index * 8;

                        draw.spr(sprite.into(), x as i32, y as i32);
                    });

                    f
                })
            })
            .collect();

        v.into()
    }
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
        );
        v.push(button);
    }

    // Draw border
    v.push(DrawFn::new(move |draw| {
        draw.palt(None);
        draw.rect(
            start_x,
            start_y,
            start_x + 4 * tile_size + 1,
            start_y + 4 * tile_size + 1,
            0,
        );
        draw.palt(Some(0));
    }));

    // Draw highlight
    v.push(DrawFn::new(move |draw| {
        let (x, y) = coordinates(selected_color as usize);

        draw.palt(None);
        draw.rect(x, y, x + tile_size - 1, y + tile_size - 1, 0);
        draw.rect(x - 1, y - 1, x + tile_size, y + tile_size, 7);
        draw.palt(Some(0));
    }));

    Tree::new(v)
}

/// The 4 rows of sprites at the bottom of the sprite editor
fn sprite_view<'a>(
    selected_sprite: usize,
    selected_tab: usize,
    sprite_buttons: &'a mut [button::State],
    tab_buttons: &'a mut [button::State],
    y: i32,
) -> Element<'a, Msg> {
    let mut sprites: Vec<Box<dyn Widget<Msg = Msg>>> = vec![DrawFn::new(move |draw| {
        draw.palt(None);
        draw.rectfill(0, y, 127, y + 32 + 1, 0);
    })];

    let sprite_position = |sprite| {
        let index = sprite % 64;
        let row = (index / 16) as i32;
        let col = (index % 16) as i32;

        (col * 8, y + 1 + row * 8)
    };

    for (index, sprite_state) in sprite_buttons.iter_mut().enumerate() {
        let sprite = index + selected_tab * 64;

        let (x, y) = sprite_position(sprite);
        sprites.push(Button::new(
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
        ));
    }

    // Draw selected sprite highlight
    {
        // TODO: Fix (wrong highlight when switching pages)
        let (x, y) = sprite_position(selected_sprite);
        sprites.push(DrawFn::new(move |draw| {
            draw.rect(x - 1, y - 1, x + 8, y + 8, 7);
        }))
    }

    for (sprite_tab, tab_button_state) in tab_buttons.iter_mut().enumerate() {
        let base_sprite = if selected_tab == sprite_tab { 33 } else { 17 };

        let x = 96 + sprite_tab as i32 * 8;
        let y = y - 8;

        sprites.push(Button::new(
            x,
            y,
            8,
            8,
            Some(Msg::SpritePageSelected(sprite_tab)),
            tab_button_state,
            DrawFn::new(move |draw| {
                draw.palt(Some(0));
                draw.spr(base_sprite + sprite_tab, 0, 0);
            }),
        ));
    }
    Tree::new(sprites)
}

fn sprite_preview(sprite: usize, x: i32, y: i32) -> Box<dyn Widget<Msg = Msg>> {
    let spr_str = format!("{:0>3}", sprite);

    Tree::new(vec![
        spr(sprite, x, y),
        DrawFn::new(move |draw| {
            draw.rectfill(x + 9, y + 1, x + 9 + 13 - 1, y + 7, 6);
            draw.print(&spr_str, x + 10, y + 2, 13);
        }),
    ])
}

fn bottom_bar() -> Box<dyn Widget<Msg = Msg>> {
    DrawFn::new(|draw| draw.rectfill(0, 121, 127, 127, 8))
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

    flag_buttons
        .iter_mut()
        .enumerate()
        .map(|(index, button)| {
            let x = x + (SPR_SIZE + 1) * index as i32;
            let flag_on = selected_sprite_flags & (1 << index) != 0;
            let color = if flag_on { FLAG_COLORS[index] } else { 1 };

            let button_content: Element<'_, Msg> = vec![
                palt(Some(7)),
                pal(1, color),
                spr(58, 0, 0),
                pal(1, 1),
                palt(Some(0)),
            ]
            .into();

            let button: Box<dyn Widget<Msg = Msg>> = Button::new(
                x,
                y,
                5,
                5,
                Some(Msg::FlagToggled(index)),
                button,
                button_content,
            );

            button
        })
        .collect::<Vec<Element<'_, Msg>>>()
        .into()
}

fn palt(transparent_color: Option<u8>) -> Box<dyn Widget<Msg = Msg> + 'static> {
    DrawFn::new(move |draw| draw.palt(transparent_color))
}

fn pal(c0: u8, c1: u8) -> Box<dyn Widget<Msg = Msg> + 'static> {
    DrawFn::new(move |draw| draw.pal(c0, c1))
}

fn spr(sprite: usize, x: i32, y: i32) -> Box<dyn Widget<Msg = Msg>> {
    DrawFn::new(move |draw| draw.spr(sprite, x, y))
}
