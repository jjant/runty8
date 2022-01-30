use crate::SPRITE_WIDTH;

use super::Rect;

pub(super) struct Canvas {}

impl Canvas {
    const BORDER: i32 = 1;

    // From a position in pixels in the screen (like (64, 64) for the middle of the screen)
    // returns the index (x,y in 0..8) of the corresponding "pixel" in the canvas.
    pub(super) fn lookup(screen_x: i32, screen_y: i32) -> Option<(i32, i32)> {
        for x in 0..(SPRITE_WIDTH as i32) {
            for y in 0..(SPRITE_WIDTH as i32) {
                if Self::pixel_rect(x, y).contains(screen_x, screen_y) {
                    return Some((x, y));
                }
            }
        }

        None
    }

    pub(super) fn position() -> Rect {
        Rect {
            x: 8,
            y: 10,
            width: 8 * 8 + Self::BORDER * 2,
            height: 8 * 8 + Self::BORDER * 2,
        }
    }

    pub(super) fn pixel_rect(x: i32, y: i32) -> Rect {
        let canvas = Self::position();

        Rect {
            x: canvas.x + Self::BORDER + 8 * x,
            y: canvas.y + Self::BORDER + 8 * y,
            width: 8,
            height: 8,
        }
    }
}
