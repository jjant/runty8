use std::fmt::Debug;

use itertools::Itertools;

use crate::ui::{
    button::{self, Button},
    slider::{self, SliderValue},
    DrawFn, Element, Tree,
};
use runty8_core::{colors, Color};

#[derive(Copy, Clone, Debug)]
pub(crate) struct BrushSize {
    size: SliderValue,
}

impl BrushSize {
    pub(crate) fn tiny() -> Self {
        Self {
            size: SliderValue::Tiny,
        }
    }

    pub(crate) fn to_human_readable(self) -> &'static str {
        match self.size {
            SliderValue::Tiny => "1",
            SliderValue::Small => "2",
            SliderValue::Medium => "3",
            SliderValue::Large => "4",
        }
    }

    fn to_screen_size(self) -> i32 {
        match self.size {
            SliderValue::Tiny => 1,
            SliderValue::Small => 2,
            SliderValue::Medium => 3,
            SliderValue::Large => 5,
        }
    }

    fn next(self) -> Self {
        let next_size = match self.size {
            SliderValue::Tiny => SliderValue::Small,
            SliderValue::Small => SliderValue::Medium,
            SliderValue::Medium => SliderValue::Large,
            SliderValue::Large => SliderValue::Tiny,
        };

        Self { size: next_size }
    }

    pub(crate) fn iter(self) -> impl Iterator<Item = (isize, isize)> {
        let size = self.to_screen_size() as isize;
        let min = -(size as f32 / 2.0 - 0.5).floor() as isize;
        let max = size / 2;

        (min..=max).cartesian_product(min..=max)
    }
}

pub(crate) struct BrushSizeSelector<'a, Msg, F> {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) selected_color: Color,
    pub(crate) brush_size: BrushSize,
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

impl<'a, Msg: Copy + Debug + 'a, F: Fn(BrushSize) -> Msg> BrushSizeSelector<'a, Msg, F> {
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
                self.brush_size.size,
                |new_size| (self.on_press)(BrushSize { size: new_size }),
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
    brush_size: BrushSize,
    selected_color: Color,
    on_press: Msg,
) -> Element<'a, Msg> {
    let local_center_x = WIDGET_SIZE / 2;
    let local_center_y = WIDGET_SIZE / 2;

    let size = brush_size.to_screen_size();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brush_size_iter_works() {
        fn brush_size_tiny() {
            assert_eq!(BrushSize::tiny().iter().collect::<Vec<_>>(), &[(0, 0)]);
        }
        fn brush_size_small() {
            let brush_size = BrushSize {
                size: SliderValue::Small,
            };

            assert_eq!(
                brush_size.iter().collect::<Vec<_>>(),
                &[(0, 0), (0, 1), (1, 0), (1, 1)]
            );
        }

        brush_size_tiny();
        brush_size_small();
    }
}
