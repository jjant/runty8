use std::{fs::File, io::Write};

use crate::{App, Button, Color, DrawContext, Sprite, State, SPRITE_HEIGHT, SPRITE_WIDTH};

pub struct SpriteEditor {
    mouse_x: i32,
    mouse_y: i32,
    mouse_pressed: bool,
    highlighted_color: Color,
    bottom_text: String,
    selected_sprite: u8,
    cursor_sprite: &'static Sprite,
}

const CANVAS_X: i32 = 79; // end = 120
const CANVAS_Y: i32 = 10;

const SPRITES_PER_ROW: u8 = 16;

impl SpriteEditor {
    fn draw_sprite_sheet(&self, y_start: i32, draw_context: &mut DrawContext) {
        const BORDER: i32 = 1;
        const HEIGHT: i32 = 32;

        draw_context.line(0, y_start, 128, y_start, 0);

        for sprite_y in 0..4 {
            for sprite_x in 0..16 {
                let sprite_index = sprite_x + sprite_y * 16;

                draw_context.spr(
                    sprite_index,
                    (sprite_x * SPRITE_WIDTH) as i32,
                    y_start + BORDER as i32 + (sprite_y * SPRITE_WIDTH) as i32,
                )
            }
        }
        draw_context.line(
            0,
            y_start + HEIGHT + BORDER,
            128,
            y_start + HEIGHT + BORDER,
            0,
        );

        // Draw highlight of selected sprite
        // TODO: Clean this up, in particular, find a way not to repeat all of these calculations
        let selected_sprite_x = self.selected_sprite % SPRITES_PER_ROW;
        let selected_sprite_y = self.selected_sprite / SPRITES_PER_ROW;

        Rect {
            x: selected_sprite_x as i32 * SPRITE_WIDTH as i32,
            y: y_start + BORDER as i32 + (selected_sprite_y as usize * SPRITE_WIDTH) as i32,
            width: SPRITE_WIDTH as i32,
            height: SPRITE_HEIGHT as i32,
        }
        .highlight(draw_context, false, 7);
    }

    fn selected_sprite<'a>(&self, draw_context: &'a DrawContext) -> &'a Sprite {
        draw_context
            .sprite_sheet
            .get_sprite(self.selected_sprite.into())
    }
}

fn serialize(bytes: &[u8]) {
    let mut file = File::create("sprite_sheet.txt").unwrap();
    file.write_all(bytes).unwrap();
}

static MOUSE_SPRITE: &'static [Color] = &[
    0, 0, 0, 0, 0, 0, 0, 0, //
    0, 0, 0, 1, 0, 0, 0, 0, //
    0, 0, 1, 7, 1, 0, 0, 0, //
    0, 0, 1, 7, 7, 1, 0, 0, //
    0, 0, 1, 7, 7, 7, 1, 0, //
    0, 0, 1, 7, 7, 7, 7, 1, //
    0, 0, 1, 7, 7, 1, 1, 0, //
    0, 0, 0, 1, 1, 7, 1, 0, //
];

static MOUSE_TARGET_SPRITE: &'static [Color] = &[
    0, 0, 0, 1, 0, 0, 0, 0, //
    0, 0, 1, 7, 1, 0, 0, 0, //
    0, 1, 0, 0, 0, 1, 0, 0, //
    1, 7, 0, 0, 0, 7, 1, 0, //
    0, 1, 0, 0, 0, 1, 0, 0, //
    0, 0, 1, 7, 1, 0, 0, 0, //
    0, 0, 0, 1, 0, 0, 0, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, //
];

impl App for SpriteEditor {
    fn init() -> Self {
        // let mut sprite_sheet = vec![11; SPRITE_AREA * SPRITE_COUNT];

        Self {
            mouse_x: 64,
            mouse_y: 64,
            mouse_pressed: false,
            highlighted_color: 7,
            bottom_text: String::new(),
            selected_sprite: 0,
            cursor_sprite: Sprite::new(MOUSE_SPRITE),
        }
    }

    fn update(&mut self, state: &State, draw_context: &mut DrawContext) {
        self.mouse_x = state.mouse_x;
        self.mouse_y = state.mouse_y;
        self.mouse_pressed = state.mouse_pressed;
        self.bottom_text = String::new();
        self.cursor_sprite = Sprite::new(MOUSE_SPRITE);

        // Handle mouse over color palette
        for color in 0..16 {
            if color_position(color).contains(self.mouse_x, self.mouse_y) {
                self.bottom_text = format!("COLOUR {}", color);
                if self.mouse_pressed {
                    self.highlighted_color = color;
                }
                return;
            }
        }

        // Handle mouse over canvas
        if canvas_position().contains(self.mouse_x, self.mouse_y) {
            self.cursor_sprite = Sprite::new(MOUSE_TARGET_SPRITE);

            let sprite = &mut draw_context
                .sprite_sheet
                .get_sprite_mut(self.selected_sprite.into())
                .sprite;

            for x in 0..(SPRITE_WIDTH as i32) {
                for y in 0..(SPRITE_WIDTH as i32) {
                    if canvas_pixel_rect(x, y).contains(self.mouse_x, self.mouse_y) {
                        self.bottom_text = format!("X:{} Y:{}", x, y);

                        if self.mouse_pressed {
                            sprite[(x + y * 8) as usize] = self.highlighted_color;
                        }
                    }
                }
            }
        }

        // Handle mouse over sprite sheet
        if SPRITE_SHEET_AREA.contains(self.mouse_x, self.mouse_y) {
            self.bottom_text = "IN SPRITE SHEET".into();

            for x in 0..SPRITES_PER_ROW {
                // TODO: Use a const for the "4"
                for y in 0..4 {
                    let sprite_area = Rect {
                        x: x as i32 * SPRITE_WIDTH as i32,
                        y: SPRITE_SHEET_AREA.y + (y as usize * SPRITE_WIDTH) as i32,
                        width: SPRITE_WIDTH as i32,
                        height: SPRITE_HEIGHT as i32,
                    };

                    if sprite_area.contains(self.mouse_x, self.mouse_y) {
                        let sprite = x + y * SPRITES_PER_ROW;
                        self.bottom_text = format!("IN SPRITE {}", sprite);
                        if self.mouse_pressed {
                            self.selected_sprite = sprite;
                        }
                        break;
                    }
                }
            }
        }

        if state.btn(Button::X) {
            serialize(draw_context.sprite_sheet.serialize().as_bytes());

            std::process::exit(1);
        }
    }

    fn draw(&self, draw_context: &mut DrawContext) {
        #[allow(dead_code)]
        draw_context.cls();
        draw_context.rectfill(0, 0, 127, 127, 5);

        // Draw top menu bar
        draw_context.rectfill(0, 0, 127, 7, 8);

        // Draw bottom menu bar
        draw_context.rectfill(0, 121, 127, 127, 8);

        // draw canvas
        canvas_position().fill(draw_context, 0);

        for x in 0..8 {
            for y in 0..8 {
                let color = self
                    .selected_sprite(draw_context)
                    .pget(x as isize, y as isize);
                canvas_pixel_rect(x, y).fill(draw_context, color);
            }
        }

        // let tools_area = Rect {
        //     x: 0,
        //     y: canvas_position().bottom() + 1,
        //     width: 128,
        //     height: 11,
        // };
        // tools_area.fill(draw_context, 12);

        let thumbnail_area = Rect {
            x: canvas_position().right() - 2,
            y: canvas_position().bottom() + 3,
            width: 8,
            height: 8,
        };

        thumbnail_area.fill(draw_context, 9);
        draw_context.spr(
            self.selected_sprite as usize,
            thumbnail_area.x,
            thumbnail_area.y,
        );

        Rect {
            x: thumbnail_area.right() + 2,
            y: thumbnail_area.y + 1,
            width: 13,
            height: 7,
        }
        .fill(draw_context, 6);
        let selected_sprite_str = format!("{:0width$}", self.selected_sprite, width = 3);
        draw_context.print(
            &selected_sprite_str,
            thumbnail_area.right() + 3,
            thumbnail_area.y + 2,
            13,
        );

        // Draw sprite sheet
        // TODO: Remove this, just here to make sure I'm not displaying the sprite sheet incorrectly
        SPRITE_SHEET_AREA.fill(draw_context, 2);

        self.draw_sprite_sheet(SPRITE_SHEET_AREA.y, draw_context);

        // Draw color palette
        draw_context.rectfill(
            CANVAS_X,
            CANVAS_Y,
            CANVAS_X + BOX_SIZE - 1,
            CANVAS_Y + BOX_SIZE - 1,
            0,
        );

        for color in 0..16 {
            let Rect {
                x,
                y,
                width,
                height,
            } = color_position(color);

            draw_context.rectfill(x, y, x + width - 1, y + height - 1, color as u8);
        }

        color_position(self.highlighted_color).highlight(draw_context, true, 7);

        draw_context.print(&self.bottom_text, 1, 122, 2);

        // Always render the mouse last (on top of everything)
        draw_context.raw_spr(self.cursor_sprite, self.mouse_x, self.mouse_y);

        // print_debug_strings(draw_context, 10, 100);
    }
}

const SPRITE_SHEET_AREA: Rect = Rect {
    x: 0,
    y: 87,
    width: 128,
    height: 34,
};

const CANVAS_BORDER: i32 = 1;
fn canvas_position() -> Rect {
    Rect {
        x: 8,
        y: 10,
        width: 8 * 8 + CANVAS_BORDER * 2,
        height: 8 * 8 + CANVAS_BORDER * 2,
    }
}

fn canvas_pixel_rect(x: i32, y: i32) -> Rect {
    let canvas = canvas_position();

    Rect {
        x: canvas.x + CANVAS_BORDER + 8 * x,
        y: canvas.y + CANVAS_BORDER + 8 * y,
        width: 8,
        height: 8,
    }
}
const SIZE: i32 = 10;
const BOX_SIZE: i32 = 4 * SIZE + 2;

#[derive(Debug)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    pub fn contains(&self, x: i32, y: i32) -> bool {
        let contains_x = x >= self.x && x < self.x + self.width;
        let contains_y = y >= self.y && y < self.y + self.height;

        contains_x && contains_y
    }

    pub fn fill(&self, draw_context: &mut DrawContext, color: Color) {
        draw_context.rectfill(
            self.x,
            self.y,
            self.x + self.width - 1,
            self.y + self.height - 1,
            color,
        )
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.height - 1
    }

    pub fn right(&self) -> i32 {
        self.x + self.width - 1
    }

    pub fn highlight(
        &self,
        draw_context: &mut DrawContext,
        include_inner: bool,
        highlight_color: Color,
    ) {
        let Rect {
            x,
            y,
            width,
            height,
        } = *self;

        if include_inner {
            draw_context.rect(x, y, x + width - 1, y + height - 1, 0)
        };
        draw_context.rect(x - 1, y - 1, x + width, y + height, highlight_color);
    }
}

fn color_position(color: Color) -> Rect {
    let x = CANVAS_X + 1 + (color as i32 % 4) * SIZE;
    let y = CANVAS_Y + 1 + (color as i32 / 4) * SIZE;

    Rect {
        x,
        y,
        width: SIZE,
        height: SIZE,
    }
}

//// DEBUG STUFF

#[allow(dead_code)]
fn print_debug_strings(draw_context: &mut DrawContext, x: i32, y: i32) {
    draw_context.print(" !\"#$%&'()*+,-.", x, y - 10, 7);
    draw_context.print("0123456789:;<=>?@", x, y, 7);
    let mut letters = "abcdefghijklmnopqrstuvwxyz".to_owned();
    letters.make_ascii_uppercase();
    draw_context.print(&letters, x, y + 10, 7);
}
