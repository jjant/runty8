use super::brush_size::{self, BrushSize, BrushSizeSelector};
use crate::runtime::sprite_sheet::{Sprite, SpriteSheet};
use crate::ui::{
    button::{self, Button},
    DrawFn, Element, Tree,
};
use crate::Color;
use itertools::Itertools;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Msg {
    ColorSelected(Color),
}

#[derive(Debug)]
pub(crate) struct Editor {
    selected_color: Color,
    color_selector_state: Vec<button::State>,
    flag_buttons: Vec<button::State>,
    pixel_buttons: Vec<button::State>,
}

impl Editor {
    pub(crate) fn new() -> Self {
        Self {
            selected_color: 0,
            color_selector_state: vec![button::State::new(); 16],
            flag_buttons: vec![button::State::new(); 8],
            pixel_buttons: vec![button::State::new(); Sprite::WIDTH * Sprite::HEIGHT],
        }
    }
    pub(crate) fn update(&mut self, msg: Msg) {
        match msg {
            Msg::ColorSelected(selected_color) => {
                self.selected_color = selected_color as u8;
            }
        }
    }

    pub(crate) fn view<'a, 'b>(
        &'a mut self,
        selected_sprite_flags: u8,
        selected_sprite: &'b Sprite,
        editor_sprites: &'a SpriteSheet,
        brush_size: BrushSize,
        brush_size_state: &'a mut brush_size::State,
        to_editor_msg: &(impl Fn(Msg) -> super::Msg + Copy),
    ) -> Element<'a, super::Msg> {
        Tree::new()
            .push(color_selector(
                79,
                10,
                10,
                self.selected_color,
                &mut self.color_selector_state,
                move |color| to_editor_msg(Msg::ColorSelected(color)),
                super::Msg::ColorHovered,
            ))
            .push(canvas_view(
                7,
                10,
                self.selected_color,
                &mut self.pixel_buttons,
                selected_sprite,
            ))
            .push(flags(
                selected_sprite_flags,
                78,
                70,
                &mut self.flag_buttons,
                editor_sprites,
            ))
            .push(
                BrushSizeSelector {
                    x: 79,
                    y: 55,
                    brush_size,
                    selected_color: self.selected_color,
                    on_press: super::Msg::BrushSizeSelected,
                    on_hover: super::Msg::BrushSizeSliderHovered,
                    state: brush_size_state,
                }
                .view(),
            )
            // .push(slider::view(93, 52, brush_size))
            .into()
    }
}

fn color_selector<'a, Msg: Debug + Copy + 'a>(
    start_x: i32,
    start_y: i32,
    tile_size: i32,
    selected_color: u8,
    states: &'a mut [button::State],
    on_press: impl (Fn(Color) -> Msg) + Copy,
    on_hover: impl (Fn(Color) -> Msg) + Copy,
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
            Some(on_press(index as Color)),
            state,
            DrawFn::new(move |draw| {
                draw.palt(None);
                draw.rectfill(0, 0, tile_size - 1, tile_size - 1, index as u8);
                draw.palt(Some(0));
            }),
        )
        .event_on_press()
        .on_hover(on_hover(index as Color))
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

#[allow(clippy::too_many_arguments)]

// TODO:
// - Change color of highlight
fn flags<'a>(
    selected_sprite_flags: u8,
    x: i32,
    y: i32,
    flag_buttons: &'a mut [button::State],
    _editor_sprites: &'a SpriteSheet,
) -> Element<'a, super::Msg> {
    const SPR_SIZE: i32 = 5;
    const FLAG_COLORS: [u8; 8] = [8, 9, 10, 11, 12, 13, 14, 15];

    let children = flag_buttons
        .iter_mut()
        .enumerate()
        .map(|(index, button)| {
            let x = x + (SPR_SIZE + 1) * index as i32;
            let flag_on = selected_sprite_flags & (1 << index) != 0;
            let color = if flag_on { FLAG_COLORS[index] } else { 1 };

            let button_content: Element<'a, super::Msg> = Tree::new()
                .push(DrawFn::new(move |pico8| {
                    pico8.palt(Some(7));
                    pico8.pal(1, color);
                    // TODO: Use the editor sprite sheet (not doing so currently,
                    // because it's still WIP).
                    //
                    // pico8.spr_from(editor_sprites, 58, 0, 0);
                    pico8.spr(58, 0, 0);
                    pico8.pal(1, 1);
                    pico8.palt(Some(0));
                }))
                .into();

            let button = Button::new(
                x,
                y,
                5,
                5,
                Some(super::Msg::FlagToggled(index)),
                button,
                button_content,
            );

            button.into()
        })
        .collect();

    Tree::with_children(children).into()
}

fn canvas_view<'a, 'b>(
    x: i32,
    y: i32,
    selected_color: Color,
    pixel_buttons: &'a mut [button::State],
    sprite: &'b Sprite,
) -> Element<'a, super::Msg> {
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
                    Some(super::Msg::SpriteEdited {
                        x: x_index,
                        y: y_index,
                        color: selected_color,
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
