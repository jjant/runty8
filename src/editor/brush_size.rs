use std::fmt::Debug;

use crate::{
    runtime::{draw_context::colors, sprite_sheet::Color},
    ui::{
        slider::{self, SliderValue},
        DrawFn, Element, Tree,
    },
};

fn slider_to_screen_size(value: SliderValue) -> i32 {
    match value {
        SliderValue::Tiny => 1,
        SliderValue::Small => 2,
        SliderValue::Medium => 3,
        SliderValue::Large => 5,
    }
}
const WIDGET_SIZE: i32 = 7;

pub(crate) struct BrushSize<'a, Msg, F> {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) selected_color: Color,
    pub(crate) brush_size: SliderValue,
    pub(crate) slider_state: &'a mut slider::State,
    pub(crate) on_press: F,
    pub(crate) on_hover: Msg,
}

impl<'a, Msg: Copy + Debug + 'a, F: Fn(SliderValue) -> Msg> BrushSize<'a, Msg, F> {
    pub(crate) fn view(self) -> Element<'a, Msg> {
        Tree::new()
            .push(size_indicator(
                self.x,
                self.y,
                self.brush_size,
                self.selected_color,
            ))
            .push(slider::view(
                self.x + 14,
                self.y - 3,
                self.brush_size,
                self.on_press,
                self.on_hover,
                self.slider_state,
            ))
            .into()
    }
}

fn size_indicator<'a, Msg: Copy + Debug + 'a>(
    x: i32,
    y: i32,
    brush_size: SliderValue,
    selected_color: Color,
) -> Element<'a, Msg> {
    let local_center_x = WIDGET_SIZE / 2;
    let local_center_y = WIDGET_SIZE / 2;

    let size = slider_to_screen_size(brush_size);

    let local_left = local_center_x - (size - 1) / 2;
    let local_top = local_center_y - (size - 1) / 2;
    let global_left = local_left + x;
    let global_top = local_top + y;

    DrawFn::new(move |draw| {
        draw.palt(None);
        draw.rectfill(
            x,
            y,
            x + WIDGET_SIZE - 1,
            y + WIDGET_SIZE - 1,
            background_color(selected_color),
        );
        draw.rectfill(
            global_left,
            global_top,
            global_left + size - 1,
            global_top + size - 1,
            selected_color,
        );
        draw.palt(Some(0));
    })
    .into()
}

fn background_color(selected_color: Color) -> Color {
    if selected_color == colors::BLACK {
        colors::LIGHT_GREY
    } else {
        colors::BLACK
    }
}
