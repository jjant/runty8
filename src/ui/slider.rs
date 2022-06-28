use std::fmt::Debug;

use super::{
    button::{self, Button},
    DrawFn, Element, Tree,
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum SliderValue {
    Tiny,
    Small,
    Medium,
    Large,
}

impl SliderValue {
    pub(crate) fn to_human_readable(self) -> &'static str {
        match self {
            SliderValue::Tiny => "1",
            SliderValue::Small => "2",
            SliderValue::Medium => "3",
            SliderValue::Large => "4",
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Tiny,
            1 => Self::Small,
            2 => Self::Medium,
            _ => Self::Large,
        }
    }

    fn to_index(self) -> i32 {
        match self {
            SliderValue::Tiny => 0,
            SliderValue::Small => 1,
            SliderValue::Medium => 2,
            SliderValue::Large => 3,
        }
    }
}

#[derive(Debug)]
pub(crate) struct State {
    button_tiny: button::State,
    button_small: button::State,
    button_medium: button::State,
    button_large: button::State,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            button_tiny: button::State::new(),
            button_small: button::State::new(),
            button_medium: button::State::new(),
            button_large: button::State::new(),
        }
    }
}

pub(crate) fn view<'a, Msg: Debug + Copy + 'a>(
    x: i32,
    y: i32,
    value: SliderValue,
    on_press: impl Fn(SliderValue) -> Msg,
    on_hover: Msg,
    state: &'a mut State,
) -> Element<'a, Msg> {
    let buttons = [
        &mut state.button_tiny,
        &mut state.button_small,
        &mut state.button_medium,
        &mut state.button_large,
    ]
    .into_iter()
    .enumerate()
    .map(move |(index, button_state)| {
        let width = 8;
        let height = 7;
        Button::new(
            x + (index as i32) * width - width / 2,
            y + 2,
            width,
            height,
            Some(on_press(SliderValue::from_index(index))),
            button_state,
            Tree::new(),
        )
        .on_hover(on_hover)
        .event_on_press()
        .into()
    })
    .collect();

    Tree::with_children(buttons)
        .push(DrawFn::new(move |draw| {
            // TODO: Use spr_ when width and height parameters are implemented.
            draw.spr(64, x, y);
            draw.spr(65, x + 8, y);
            draw.spr(66, x + 16, y);
            draw.spr(67, x + 24, y);

            // Draw selection indicator
            draw.spr(68, x + value.to_index() * 8 - 2, y);
        }))
        .into()
}
