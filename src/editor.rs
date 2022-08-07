mod brush_size;
pub mod key_combo;
mod notification;
pub mod serialize;
mod sound_editor;
mod undo_redo;

use crate::app::ElmApp;
use crate::editor::notification::Notification;
use crate::runtime::flags::Flags;
use crate::runtime::map::Map;
use crate::runtime::sprite_sheet::{Color, Sprite, SpriteSheet};
use crate::ui::button::{self, Button};
use crate::ui::{
    cursor::{self, Cursor},
    text::Text,
};
use crate::ui::{DrawFn, Element, Tree};
use crate::Resources;
use crate::{Event, Key, KeyState, KeyboardEvent};
use itertools::Itertools;
use serialize::serialize;

use self::brush_size::{BrushSize, BrushSizeSelector};
use self::key_combo::KeyCombos;
use self::serialize::{Ppm, Serialize};
use self::undo_redo::{Command, Commands};

#[derive(Debug)]
pub(crate) struct Editor {
    cursor: cursor::State,
    tab: Tab,
    selected_color: u8,
    selected_sprite: usize,
    selected_sprite_page: usize,
    sprite_button_state: button::State,
    map_button_state: button::State,
    sound_button_state: button::State,
    tab_buttons: [button::State; 4],
    color_selector_state: Vec<button::State>,
    flag_buttons: Vec<button::State>,
    sprite_buttons: Vec<button::State>,
    pixel_buttons: Vec<button::State>,
    selected_tool: usize,
    tool_buttons: Vec<button::State>,
    bottom_bar_text: String,
    map_buttons: Vec<button::State>,
    hovered_map_button: usize,
    show_sprites_in_map: bool,
    notification: notification::State,
    key_combos: KeyCombos<KeyComboAction>,
    clipboard: Clipboard,
    commands: Commands,
    brush_size: BrushSize,
    brush_size_state: brush_size::State,
    editor_sprites: SpriteSheet,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Tab {
    // ^ pub(crate) required because of Msg::ChangedTab
    // We may want to fix that later (that is, we want this type to actually be private).
    SpriteEditor,
    MapEditor,
    SoundEditor,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Msg {
    ChangedTab { new_tab: Tab },
    ColorSelected(usize),
    ColorHovered(usize),
    SpritePageSelected(usize),
    SpriteButtonClicked(usize),
    FlagToggled(usize),
    SpriteEdited { x: usize, y: usize }, // TODO: Improve
    ToolSelected(usize),
    MapSpriteHovered(usize),
    ClickedMapTile { x: usize, y: usize },
    KeyboardEvent(KeyboardEvent),
    BrushSizeSliderHovered,
    BrushSizeSelected(BrushSize),
}

impl Editor {
    fn switch_map_mode(&mut self) {
        self.show_sprites_in_map = !self.show_sprites_in_map;
    }

    fn shift_sprite(&mut self, shift_direction: ShiftDirection, sprite_sheet: &mut SpriteSheet) {
        let sprite = sprite_sheet.get_sprite_mut(self.selected_sprite);
        shift_direction.shift(sprite);
    }

    fn handle_key_combos(&mut self, key_event: KeyboardEvent, resources: &mut Resources) {
        self.key_combos.on_event(key_event, |action| {
            handle_key_combo(
                action,
                self.selected_sprite,
                &mut self.notification,
                &mut self.clipboard,
                resources,
                &mut self.commands,
            );
        })
    }
}

#[derive(Debug)]
struct Clipboard {
    data: Vec<Color>,
}
impl Clipboard {
    fn new() -> Self {
        Self { data: vec![0; 64] }
    }

    fn copy_sprite(&mut self, sprite: &Sprite) {
        self.data = sprite.to_owned();
    }

    fn paste_into(&self, sprite: &mut Sprite) {
        for (sprite_pixel, clipboard_pixel) in sprite.iter_mut().zip(self.data.iter().copied()) {
            *sprite_pixel = clipboard_pixel;
        }
    }
}

fn handle_key_combo(
    key_combo: KeyComboAction,
    selected_sprite: usize,
    notification: &mut notification::State,
    clipboard: &mut Clipboard,
    resources: &mut Resources,
    commands: &mut Commands,
) {
    match key_combo {
        KeyComboAction::Copy => {
            let sprite = resources.sprite_sheet.get_sprite(selected_sprite);
            notification.alert("COPIED 1 X 1 SPRITES".to_owned());
            clipboard.copy_sprite(sprite);
        }
        KeyComboAction::Paste => {
            let sprite = resources.sprite_sheet.get_sprite_mut(selected_sprite);
            notification.alert("PASTED 1 X 1 SPRITES".to_owned());

            clipboard.paste_into(sprite);
        }
        KeyComboAction::FlipVertically => {
            let sprite = resources.sprite_sheet.get_sprite_mut(selected_sprite);

            sprite.flip_vertically()
        }
        KeyComboAction::FlipHorizontally => {
            let sprite = resources.sprite_sheet.get_sprite_mut(selected_sprite);

            sprite.flip_horizontally()
        }
        KeyComboAction::Undo => {
            commands.undo(notification, &mut resources.sprite_sheet);
        }
        KeyComboAction::Redo => {
            commands.redo(notification, &mut resources.sprite_sheet);
        }
        KeyComboAction::Save => {
            save(notification, resources);
        }
    }
}

fn save(notification: &mut notification::State, resources: &Resources) {
    notification.alert("SAVED".to_owned());

    let map_ppm = Ppm::from_map(&resources.map, &resources.sprite_sheet);
    let sprite_sheet_ppm = Ppm::from_sprite_sheet(&resources.sprite_sheet);
    let to_serialize: &[(&str, &dyn Serialize)] = &[
        (&Flags::file_name(), &resources.sprite_flags),
        (&SpriteSheet::file_name(), &resources.sprite_sheet),
        (&Map::file_name(), &resources.map),
        ("map.ppm", &map_ppm),
        ("sprite_sheet.ppm", &sprite_sheet_ppm),
    ];

    for (name, serializable) in to_serialize.iter() {
        serialize(&resources.assets_path, name, serializable);
    }
}

#[derive(Copy, Clone, Debug)]
enum KeyComboAction {
    Copy,
    Paste,
    FlipVertically,
    FlipHorizontally,
    Undo,
    Redo,
    Save,
}

fn load_editor_sprite_sheet() -> Result<SpriteSheet, String> {
    let editor_sprites = std::fs::read_to_string("./src/editor/sprite_sheet.txt")
        .map_err(|_| "Couldn't find editor sprite sheet file.".to_owned())?;

    SpriteSheet::deserialize(&editor_sprites)
        .map_err(|_| "Couldn't parse editor sprite sheet.".to_owned())
}

impl ElmApp for Editor {
    type Msg = Msg;

    fn init() -> Self {
        let selected_sprite = 0;
        Self {
            cursor: cursor::State::new(),
            sprite_button_state: button::State::new(),
            map_button_state: button::State::new(),
            sound_button_state: button::State::new(),
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
            pixel_buttons: vec![button::State::new(); Sprite::WIDTH * Sprite::HEIGHT],
            selected_tool: 0,
            tool_buttons: vec![button::State::new(); 2],
            bottom_bar_text: "".to_owned(),
            map_buttons: vec![button::State::new(); 144],
            hovered_map_button: 0,
            show_sprites_in_map: false,
            notification: notification::State::new(),
            key_combos: KeyCombos::new()
                .push(KeyComboAction::Copy, Key::C, &[Key::Control])
                .push(KeyComboAction::Paste, Key::V, &[Key::Control])
                .push(KeyComboAction::Undo, Key::Z, &[Key::Control])
                .push(KeyComboAction::Redo, Key::Y, &[Key::Control])
                .push(KeyComboAction::Save, Key::S, &[Key::Control])
                .push(KeyComboAction::FlipVertically, Key::V, &[])
                .push(KeyComboAction::FlipHorizontally, Key::F, &[]),
            clipboard: Clipboard::new(),
            commands: Commands::new(),
            brush_size: BrushSize::tiny(),
            brush_size_state: brush_size::State::new(),
            editor_sprites: load_editor_sprite_sheet()
                // TODO: Change this to actually crash if it failed.
                .unwrap_or_else(|_| SpriteSheet::new()),
        }
    }

    fn update(&mut self, msg: &Msg, resources: &mut Resources) {
        match msg {
            &Msg::KeyboardEvent(event) => {
                self.handle_key_combos(event, resources);

                match event {
                    KeyboardEvent {
                        key: Key::C,
                        state: KeyState::Down,
                    } => self.switch_map_mode(),

                    KeyboardEvent {
                        key,
                        state: KeyState::Down,
                    } => {
                        if let Some(shift_direction) = ShiftDirection::from_key(&key) {
                            self.shift_sprite(shift_direction, &mut resources.sprite_sheet)
                        }
                    }
                    _ => {}
                }
            }
            &Msg::ChangedTab { new_tab } => {
                self.tab = new_tab;
                println!("Changed tab: {:?}", new_tab);
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

                let flag_value = resources
                    .sprite_flags
                    .fget_n(self.selected_sprite, flag_index as u8);
                resources
                    .sprite_flags
                    .fset(self.selected_sprite, flag_index, !flag_value);
            }
            &Msg::SpriteEdited { x, y } => {
                let sprite = resources.sprite_sheet.get_sprite_mut(self.selected_sprite);
                let x = x as isize;
                let y = y as isize;
                let previous_color = sprite.pget(x, y);

                self.commands.push(Command::pixel_changed(
                    self.selected_sprite,
                    x,
                    y,
                    previous_color,
                    self.selected_color,
                ));

                for (x, y) in self
                    .brush_size
                    .iter()
                    .map(|(local_x, local_y)| (local_x + x, local_y + y))
                {
                    sprite.pset(x, y, self.selected_color);
                }
            }
            &Msg::ToolSelected(selected_tool) => {
                self.selected_tool = selected_tool;
            }
            &Msg::ColorHovered(color) => {
                self.bottom_bar_text = format!("COLOUR {}", color);
            }

            &Msg::MapSpriteHovered(sprite) => {
                self.hovered_map_button = sprite;
            }
            &Msg::ClickedMapTile { x, y } => {
                resources.map.mset(x, y, self.selected_sprite as u8);
            }
            &Msg::BrushSizeSelected(brush_size) => {
                self.brush_size = brush_size;
                self.bottom_bar_text =
                    format!("BRUSH SIZE: {}", self.brush_size.to_human_readable());
            }
            &Msg::BrushSizeSliderHovered => {
                self.bottom_bar_text =
                    format!("BRUSH SIZE: {}", self.brush_size.to_human_readable());
            }
        }
    }

    fn view(&mut self, resources: &Resources) -> Element<'_, Msg> {
        const BACKGROUND: u8 = 5;

        Tree::new()
            .push(DrawFn::new(|draw| {
                draw.rectfill(0, 0, 127, 127, BACKGROUND)
            }))
            .push(top_bar(
                &mut self.sprite_button_state,
                &mut self.map_button_state,
                &mut self.sound_button_state,
                self.tab,
            ))
            .push(match self.tab {
                Tab::SpriteEditor => sprite_editor_view(
                    self.selected_color,
                    &mut self.color_selector_state,
                    resources.sprite_flags.get(self.selected_sprite).unwrap(),
                    resources.sprite_sheet.get_sprite(self.selected_sprite),
                    &mut self.flag_buttons,
                    &mut self.pixel_buttons,
                    self.brush_size,
                    &mut self.brush_size_state,
                    &self.editor_sprites,
                ),
                Tab::MapEditor => Tree::new()
                    .push(map_view(
                        &resources.map,
                        0,
                        8,
                        &mut self.map_buttons,
                        self.hovered_map_button,
                        self.show_sprites_in_map,
                    ))
                    .into(),
                Tab::SoundEditor => Tree::new().push(sound_editor::view()).into(),
            })
            .push(tools_row(
                76,
                self.selected_sprite,
                self.selected_sprite_page,
                &mut self.tab_buttons,
                self.selected_tool,
                &mut self.tool_buttons,
            ))
            .push(sprite_view(
                self.selected_sprite,
                self.selected_sprite_page,
                &mut self.sprite_buttons,
                87,
            ))
            .push(bottom_bar(&self.bottom_bar_text))
            .push(Cursor::new(&mut self.cursor))
            .push(Notification::new(&mut self.notification))
            .into()
    }

    fn subscriptions(&self, event: &Event) -> Vec<Msg> {
        match event {
            Event::Mouse(_) => None,
            Event::Keyboard(event) => Some(Msg::KeyboardEvent(*event)),
            Event::Tick { .. } => None,
        }
        .into_iter()
        .collect()
    }
}

fn top_bar<'a>(
    sprite_button_state: &'a mut button::State,
    map_button_state: &'a mut button::State,
    sound_button_state: &'a mut button::State,
    tab: Tab,
) -> Element<'a, Msg> {
    Tree::new()
        .push(DrawFn::new(|draw| {
            draw.rectfill(0, 0, 127, 7, 8);
        }))
        .push(sprite_editor_button(sprite_button_state, tab))
        .push(map_editor_button(map_button_state, tab))
        .push(sound_editor_button(sound_button_state, tab))
        .into()
}

#[allow(clippy::too_many_arguments)]
fn sprite_editor_view<'a, 'b>(
    selected_color: u8,
    color_selector_state: &'a mut [button::State],
    selected_sprite_flags: u8,
    selected_sprite: &'b Sprite,
    flag_buttons: &'a mut [button::State],
    pixel_buttons: &'a mut [button::State],
    brush_size: BrushSize,
    brush_size_state: &'a mut brush_size::State,
    editor_sprites: &'a SpriteSheet,
) -> Element<'a, Msg> {
    Tree::new()
        .push(color_selector(
            79,
            10,
            10,
            selected_color,
            color_selector_state,
            Msg::ColorSelected,
            Msg::ColorHovered,
        ))
        .push(canvas_view(7, 10, pixel_buttons, selected_sprite))
        .push(flags(
            selected_sprite_flags,
            78,
            70,
            flag_buttons,
            editor_sprites,
        ))
        .push(
            BrushSizeSelector {
                x: 79,
                y: 55,
                brush_size,
                selected_color,
                on_press: Msg::BrushSizeSelected,
                on_hover: Msg::BrushSizeSliderHovered,
                state: brush_size_state,
            }
            .view(),
        )
        // .push(slider::view(93, 52, brush_size))
        .into()
}

fn map_view<'a, 'b>(
    map: &'a Map,
    x: i32,
    y: i32,
    map_buttons: &'b mut [button::State],
    hovered_map_button: usize,
    show_sprites_in_map: bool,
) -> Element<'b, Msg> {
    let mut v: Vec<Element<'_, Msg>> = map_buttons
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
                    Some(Msg::ClickedMapTile {
                        x: col_index,
                        y: row_index,
                    }),
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
                .on_hover(Msg::MapSpriteHovered(col_index + row_index * 16))
                .into()
            })
        })
        .collect();

    // Draw highlight
    let col_index = hovered_map_button % 16;
    let row_index = hovered_map_button / 16;
    let x = x as usize + col_index * 8;
    let y = y as usize + row_index * 8;
    v.push(
        DrawFn::new(move |draw| draw.rect(x as i32, y as i32, (x + 7) as i32, (y + 7) as i32, 7))
            .into(),
    );

    Tree::with_children(v).into()
}

fn sprite_editor_button(state: &mut button::State, tab: Tab) -> Element<'_, Msg> {
    let selected = tab == Tab::SpriteEditor;

    editor_button(
        state,
        Icons::SpriteEditor.to_raw(),
        102,
        0,
        Msg::ChangedTab {
            new_tab: Tab::SpriteEditor,
        },
        selected,
    )
}

fn map_editor_button(state: &mut button::State, tab: Tab) -> Element<'_, Msg> {
    let selected = tab == Tab::MapEditor;

    editor_button(
        state,
        Icons::MapEditor.to_raw(),
        110,
        0,
        Msg::ChangedTab {
            new_tab: Tab::MapEditor,
        },
        selected,
    )
}

fn sound_editor_button(state: &mut button::State, tab: Tab) -> Element<'_, Msg> {
    let selected = tab == Tab::SoundEditor;

    editor_button(
        state,
        Icons::SoundEditor.to_raw(),
        118,
        0,
        Msg::ChangedTab {
            new_tab: Tab::SoundEditor,
        },
        selected,
    )
}

fn editor_button(
    state: &mut button::State,
    sprite: usize,
    x: i32,
    y: i32,
    msg: Msg,
    selected: bool,
) -> Element<'_, Msg> {
    Button::new(
        x,
        y,
        8,
        8,
        Some(msg),
        state,
        DrawFn::new(move |draw| {
            let color = if selected { 15 } else { 2 };

            draw.pal(15, color);
            draw.spr(sprite, 0, 0);
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
    on_hover: impl (Fn(usize) -> Msg) + Copy + 'static,
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
        .event_on_press()
        .on_hover(on_hover(index))
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

fn tools_row<'a>(
    y: i32,
    sprite: usize,
    selected_tab: usize,
    tab_buttons: &'a mut [button::State],
    selected_tool: usize,
    tool_buttons: &'a mut [button::State],
) -> Element<'a, Msg> {
    let mut children = vec![DrawFn::new(move |draw| {
        const HEIGHT: i32 = 11;
        draw.rectfill(0, y, 127, y + HEIGHT - 1, 5)
    })
    .into()];

    const TOOLS: &[usize] = &[15, 31];

    for (tool_index, tool_button) in tool_buttons.iter_mut().enumerate() {
        let spr = TOOLS[tool_index];

        let x = (9 + 8 * tool_index) as i32;
        let y = y + 2;
        children.push(
            Button::new(
                x,
                y,
                8,
                8,
                Some(Msg::ToolSelected(tool_index)),
                tool_button,
                DrawFn::new(move |draw| {
                    if selected_tool == tool_index {
                        draw.pal(13, 7);
                    }
                    draw.spr(spr, 0, 0);
                    draw.pal(13, 13);
                }),
            )
            .into(),
        );
    }

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
            .event_on_press()
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

fn bottom_bar(text: &str) -> Element<'_, Msg> {
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
fn flags<'a>(
    selected_sprite_flags: u8,
    x: i32,
    y: i32,
    flag_buttons: &'a mut [button::State],
    _editor_sprites: &'a SpriteSheet,
) -> Element<'a, Msg> {
    const SPR_SIZE: i32 = 5;
    const FLAG_COLORS: [u8; 8] = [8, 9, 10, 11, 12, 13, 14, 15];

    let children = flag_buttons
        .iter_mut()
        .enumerate()
        .map(|(index, button)| {
            let x = x + (SPR_SIZE + 1) * index as i32;
            let flag_on = selected_sprite_flags & (1 << index) != 0;
            let color = if flag_on { FLAG_COLORS[index] } else { 1 };

            let button_content: Element<'a, Msg> = Tree::new()
                .push(palt(Some(7)))
                .push(pal(1, color))
                .push(DrawFn::new(|pico8| {
                    // TODO: Use the editor sprite sheet (not doing so currently,
                    // because it's still WIP).
                    //
                    // pico8.spr_from(editor_sprites, 58, 0, 0);
                    pico8.spr(58, 0, 0);
                }))
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

fn palt<'a>(transparent_color: Option<u8>) -> impl Into<Element<'a, Msg>> {
    DrawFn::new(move |draw| draw.palt(transparent_color))
}

fn pal<'a>(c0: u8, c1: u8) -> impl Into<Element<'a, Msg>> {
    DrawFn::new(move |draw| draw.pal(c0, c1))
}

fn spr<'a>(sprite: usize, x: i32, y: i32) -> impl Into<Element<'a, Msg>> {
    DrawFn::new(move |draw| draw.spr(sprite, x, y))
}

fn canvas_view<'a, 'b>(
    x: i32,
    y: i32,
    pixel_buttons: &'a mut [button::State],
    sprite: &'b Sprite,
) -> Element<'a, Msg> {
    let mut elements = vec![];

    for (y_index, chunk) in pixel_buttons
        .iter_mut()
        .zip(sprite.iter())
        .chunks(8)
        .into_iter()
        .enumerate()
    {
        let y = y + 1 + (y_index * Sprite::HEIGHT) as i32;
        for (x_index, (button, pixel_color)) in chunk.enumerate() {
            let x = x + 1 + (x_index * Sprite::WIDTH) as i32;

            elements.push(
                Button::new(
                    x,
                    y,
                    Sprite::WIDTH as i32,
                    Sprite::HEIGHT as i32,
                    Some(Msg::SpriteEdited {
                        x: x_index,
                        y: y_index,
                    }),
                    button,
                    DrawFn::new(move |draw| {
                        draw.palt(None);
                        draw.rectfill(0, 0, 7, 7, pixel_color);
                    }),
                )
                .event_on_press()
                .into(),
            )
        }
    }

    let highlight = DrawFn::new(move |draw| {
        draw.palt(None);
        draw.rect(x, y, x + 64 + 1, y + 64 + 1, 0)
    });

    Tree::with_children(elements).push(highlight).into()
}

pub(crate) static MOUSE_SPRITE: &[Color] = &[
    0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 1, 0, 0, 0, 0, //
    0, 0, 1, 7, 1, 0, 0, 0, //
    0, 0, 1, 7, 7, 1, 0, 0, //
    0, 0, 1, 7, 7, 7, 1, 0, //
    0, 0, 1, 7, 7, 7, 7, 1, //
    0, 0, 1, 7, 7, 1, 1, 0, //
    0, 0, 0, 1, 1, 7, 1, 0, //
];

// static MOUSE_TARGET_SPRITE: &[Color] = &[
//     0, 0, 0, 1, 0, 0, 0, 0, //
//     0, 0, 1, 7, 1, 0, 0, 0, //
//     0, 1, 0, 0, 0, 1, 0, 0, //
//     1, 7, 0, 0, 0, 7, 1, 0, //
//     0, 1, 0, 0, 0, 1, 0, 0, //
//     0, 0, 1, 7, 1, 0, 0, 0, //
//     0, 0, 0, 1, 0, 0, 0, 0, //
//     0, 0, 0, 0, 0, 0, 0, 0, //
// ];

#[derive(Clone, Copy, Debug)]
enum ShiftDirection {
    Up,
    Down,
    Left,
    Right,
}

impl ShiftDirection {
    fn from_key(key: &Key) -> Option<Self> {
        use ShiftDirection::*;

        match key {
            Key::W => Some(Up),
            Key::D => Some(Right),
            Key::S => Some(Down),
            Key::A => Some(Left),
            _ => None,
        }
    }
    fn shift(&self, sprite: &mut Sprite) {
        match self {
            ShiftDirection::Up => sprite.shift_up(),
            ShiftDirection::Down => sprite.shift_down(),
            ShiftDirection::Left => sprite.shift_left(),
            ShiftDirection::Right => sprite.shift_right(),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(usize)]
enum Icons {
    MapEditor = 62,
    SpriteEditor = 63,
    SoundEditor = 60,
    // MusicEditor = 61,
}

impl Icons {
    fn to_raw(&self) -> usize {
        *self as usize
    }
}
