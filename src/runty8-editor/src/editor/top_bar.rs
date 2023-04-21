use crate::pico8::Pico8EditorExt as _;
use crate::ui::{
    button::{self, Button},
    DrawFn, Element, Tree,
};

use super::{Msg, Tab};

#[derive(Debug)]
pub(crate) struct TopBar {
    #[cfg(target_arch = "wasm32")]
    export_assets_button: button::State,
    sprite_editor_button: button::State,
    map_editor_button: button::State,
}

impl TopBar {
    pub(crate) fn new() -> Self {
        Self {
            #[cfg(target_arch = "wasm32")]
            export_assets_button: button::State::new(),
            sprite_editor_button: button::State::new(),
            map_editor_button: button::State::new(),
        }
    }

    pub(crate) fn view(&mut self, tab: Tab) -> Element<'_, Msg> {
        let buttons = vec![
            #[cfg(target_arch = "wasm32")]
            wasm::export_assets_button(&mut self.export_assets_button),
            sprite_editor_button(&mut self.sprite_editor_button, tab),
            map_editor_button(&mut self.map_editor_button, tab),
        ];

        let background = DrawFn::new(|draw| {
            draw.rectfill(0, 0, 127, 7, 8);
        });

        Tree::new().push(background).push(buttons).into()
    }
}

fn sprite_editor_button(state: &mut button::State, tab: Tab) -> Element<'_, Msg> {
    let selected = tab == Tab::SpriteEditor;

    editor_button(state, 63, 110, 0, Msg::SpriteTabClicked, selected).into()
}

fn map_editor_button(state: &mut button::State, tab: Tab) -> Element<'_, Msg> {
    let selected = tab == Tab::MapEditor;

    editor_button(state, 62, 118, 0, Msg::MapButtonClicked, selected).into()
}

fn editor_button(
    state: &mut button::State,
    sprite: usize,
    x: i32,
    y: i32,
    on_press: Msg,
    selected: bool,
) -> Button<Msg> {
    Button::new(
        x,
        y,
        8,
        8,
        Some(on_press),
        state,
        DrawFn::new(move |draw| {
            let color = if selected { 15 } else { 2 };

            draw.pal(2, color);
            draw.editor_spr(sprite, 0, 0);
            draw.pal(2, 2);
        }),
    )
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::Msg;
    use crate::ui::{button, Element};

    pub(crate) fn export_assets_button(state: &mut button::State) -> Element<'_, Msg> {
        super::editor_button(state, 61, 1, 0, Msg::ExportWebAssets, false)
            .on_hover(Msg::ExportWebAssetsHovered)
            .into()
    }
}
