use std::fmt::Debug;

use crate::{
    runtime::{draw_context::colors, sprite_sheet::Color},
    ui::{DrawFn, Element},
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum BrushSize {
    Tiny,
    Small,
    Medium,
    Large,
}

impl BrushSize {
    pub(crate) fn next_size(&self) -> Self {
        match self {
            BrushSize::Tiny => BrushSize::Small,
            BrushSize::Small => BrushSize::Medium,
            BrushSize::Medium => BrushSize::Large,
            BrushSize::Large => BrushSize::Tiny,
        }
    }

    fn to_screen_size(self) -> i32 {
        match self {
            BrushSize::Tiny => 1,
            BrushSize::Small => 2,
            BrushSize::Medium => 3,
            BrushSize::Large => 5,
        }
    }
}

const WIDGET_SIZE: i32 = 7;
pub(crate) fn view<'a, Msg: Copy + Debug + 'a>(
    x: i32,
    y: i32,
    brush_size: BrushSize,
    selected_color: Color,
) -> Element<'a, Msg> {
    let local_center_x = WIDGET_SIZE / 2;
    let local_center_y = WIDGET_SIZE / 2;

    let size = brush_size.to_screen_size();

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
