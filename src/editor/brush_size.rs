use std::fmt::Debug;

use crate::{
    runtime::{draw_context::colors, sprite_sheet::Color},
    ui::{
        button::{self, Button},
        slider::{self, SliderValue},
        DrawFn, Element, Tree,
    },
};

pub(crate) struct BrushSize<'a, Msg, F> {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) selected_color: Color,
    pub(crate) brush_size: SliderValue,
    pub(crate) on_press: F,
    pub(crate) on_hover: Msg,
    pub(crate) state: &'a mut State,
}

#[derive(Debug)]
pub(crate) struct State {
    slider_state: slider::State,
    button_state: button::State,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            slider_state: slider::State::new(),
            button_state: button::State::new(),
        }
    }
}

impl<'a, Msg: Copy + Debug + 'a, F: Fn(SliderValue) -> Msg> BrushSize<'a, Msg, F> {
    pub(crate) fn view(self) -> Element<'a, Msg> {
        Tree::new()
            .push(size_indicator(
                self.x,
                self.y,
                &mut self.state.button_state,
                self.brush_size,
                self.selected_color,
                (self.on_press)(self.brush_size.next()),
            ))
            .push(slider::view(
                self.x + 14,
                self.y - 3,
                self.brush_size,
                self.on_press,
                self.on_hover,
                &mut self.state.slider_state,
            ))
            .into()
    }
}

const WIDGET_SIZE: i32 = 7;

fn size_indicator<'a, Msg: Copy + Debug + 'a>(
    x: i32,
    y: i32,
    state: &'a mut button::State,
    brush_size: SliderValue,
    selected_color: Color,
    on_press: Msg,
) -> Element<'a, Msg> {
    let local_center_x = WIDGET_SIZE / 2;
    let local_center_y = WIDGET_SIZE / 2;

    let size = slider_to_screen_size(brush_size);

    Button::new(
        x,
        y,
        WIDGET_SIZE,
        WIDGET_SIZE,
        Some(on_press),
        state,
        DrawFn::new(move |draw| {
            let local_left = local_center_x - (size - 1) / 2;
            let local_top = local_center_y - (size - 1) / 2;

            draw.palt(None);
            draw.rectfill(
                0,
                0,
                WIDGET_SIZE - 1,
                WIDGET_SIZE - 1,
                background_color(selected_color),
            );
            draw.rectfill(
                local_left,
                local_top,
                local_left + size - 1,
                local_top + size - 1,
                selected_color,
            );
            draw.palt(Some(0));
        }),
    )
    .event_on_press()
    .into()
}

fn background_color(selected_color: Color) -> Color {
    if selected_color == colors::BLACK {
        colors::LIGHT_GREY
    } else {
        colors::BLACK
    }
}

fn slider_to_screen_size(value: SliderValue) -> i32 {
    match value {
        SliderValue::Tiny => 1,
        SliderValue::Small => 2,
        SliderValue::Medium => 3,
        SliderValue::Large => 5,
    }
}
