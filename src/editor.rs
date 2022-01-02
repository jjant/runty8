use std::{
    fs::File,
    io::{Read, Write},
};

use itertools::Itertools;

use crate::{font, App, Button, Color, DrawContext, State};

pub struct SpriteEditor {
    mouse_x: i32,
    mouse_y: i32,
    mouse_pressed: bool,
    highlighted_color: Color,
    bottom_text: String,
    sprite_sheet: Vec<Color>,
    #[allow(dead_code)]
    selected_sprite: u8,
}

const CANVAS_X: i32 = 79; // end = 120
const CANVAS_Y: i32 = 10;

const SPRITE_WIDTH: usize = 8;
const SPRITE_SIZE: usize = SPRITE_WIDTH * 8;
const SPRITES_PER_ROW: i32 = SPRITE_SHEET_WIDTH / SPRITE_WIDTH as i32;
const SPRITE_SHEET_WIDTH: i32 = 128;

impl SpriteEditor {
    fn draw_sprite_sheet(&self, y_start: i32, draw_context: &mut DrawContext) {
        const BORDER: i32 = 1;
        const HEIGHT: i32 = 32;

        draw_context.line(0, y_start, 128, y_start, 0);

        for sprite_y in 0..4 {
            for sprite_x in 0..16 {
                let sprite = sprite_x + sprite_y * 16;

                self.draw_sprite(
                    draw_context,
                    sprite,
                    (sprite_x * SPRITE_WIDTH) as i32,
                    y_start + BORDER as i32 + (sprite_y * SPRITE_WIDTH) as i32,
                );
            }
        }
        draw_context.line(
            0,
            y_start + HEIGHT + BORDER,
            128,
            y_start + HEIGHT + BORDER,
            0,
        );
    }

    // TODO: Reorganize this.
    fn draw_sprite(&self, draw_context: &mut DrawContext, sprite: usize, x: i32, y: i32) {
        let x_pos = sprite % SPRITES_PER_ROW as usize;
        let y_pos = sprite / SPRITES_PER_ROW as usize;
        let idx = x_pos * SPRITE_WIDTH + y_pos * 128 * 8;

        for j in 0..8 {
            let y_offset = j * 128;

            for i in 0..8 {
                draw_context.pset(
                    x + i as i32,
                    y + j as i32,
                    self.sprite_sheet[idx + y_offset + i],
                );
            }
        }
    }

    fn serialize(&self) -> String {
        let lines = self.sprite_sheet.chunks(128).map(|chunk| {
            Itertools::intersperse(chunk.iter().map(|n| format!("{:X}", n)), "".to_owned())
                .collect()
        });

        Itertools::intersperse(lines, "\n".to_owned()).collect::<String>()
    }
}

fn serialize(bytes: &[u8]) {
    let mut file = File::create("sprite_sheet.txt").unwrap();
    file.write_all(bytes).unwrap();
}

// TODO: Make a more reliable version of this.
fn deserialize() -> Vec<u8> {
    let mut file = Vec::with_capacity(128 * 128);
    File::open("sprite_sheet.txt")
        .expect("Couldn't read file")
        .read_to_end(&mut file)
        .unwrap();

    file.into_iter()
        .filter_map(|c| (c as char).to_digit(16))
        .map(|c| c as u8)
        .collect()
}

impl App for SpriteEditor {
    fn init() -> Self {
        // let mut sprite_sheet = vec![11; SPRITE_SIZE * SPRITE_COUNT];
        let sprite_sheet = deserialize();

        Self {
            mouse_x: 64,
            mouse_y: 64,
            mouse_pressed: false,
            highlighted_color: 7,
            bottom_text: String::new(),
            sprite_sheet,
            selected_sprite: 0,
        }
    }

    fn update(&mut self, state: &State) {
        self.mouse_x = state.mouse_x;
        self.mouse_y = state.mouse_y;
        self.mouse_pressed = state.mouse_pressed;
        self.bottom_text = String::new();

        for color in 0..16 {
            if color_position(color).contains(self.mouse_x, self.mouse_y) {
                self.bottom_text = format!("COLOUR {}", color);
                if self.mouse_pressed {
                    self.highlighted_color = color;
                }
                return;
            }
        }

        if canvas_position().contains(self.mouse_x, self.mouse_y) {
            for x in 0..(SPRITE_SIZE as i32) {
                for y in 0..(SPRITE_SIZE as i32) {
                    if canvas_pixel_rect(x, y).contains(self.mouse_x, self.mouse_y) {
                        self.bottom_text = format!("X:{} Y:{}", x, y);
                        if self.mouse_pressed {
                            self.sprite_sheet[(x + y * 128) as usize] = self.highlighted_color;
                        }
                    }
                }
            }
        }

        if state.btn(Button::X) {
            serialize(self.serialize().as_bytes());

            std::process::exit(1);
        }
    }

    fn draw(&self, draw_context: &mut DrawContext) {
        const MOUSE_SPRITE: [u8; 8 * 8] = [
            0, 1, 0, 0, 0, 0, 0, 0, //
            1, 7, 1, 0, 0, 0, 0, 0, //
            1, 7, 7, 1, 0, 0, 0, 0, //
            1, 7, 7, 7, 1, 0, 0, 0, //
            1, 7, 7, 7, 7, 1, 0, 0, //
            1, 7, 7, 1, 1, 0, 0, 0, //
            0, 1, 1, 7, 1, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
        ];

        #[allow(dead_code)]
        const MOUSE_TARGET_SPRITE: [u8; SPRITE_SIZE] = [
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 1, 0, 0, 0, 0, //
            0, 0, 1, 7, 1, 0, 0, 0, //
            0, 1, 0, 0, 0, 1, 0, 0, //
            1, 7, 0, 0, 0, 7, 1, 0, //
            0, 1, 0, 0, 0, 1, 0, 0, //
            0, 0, 1, 7, 1, 0, 0, 0, //
            0, 0, 0, 1, 0, 0, 0, 0, //
        ];

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
                canvas_pixel_rect(x, y).fill(
                    draw_context,
                    self.sprite_sheet[(x + y * SPRITE_SHEET_WIDTH) as usize],
                )
            }
        }

        let tools_area = Rect {
            x: 0,
            y: canvas_position().bottom() + 1,
            width: 128,
            height: 11,
        };
        tools_area.fill(draw_context, 12);

        let thumbnail_area = Rect {
            x: canvas_position().right() - 2,
            y: canvas_position().bottom() + 3,
            width: 8,
            height: 8,
        };

        thumbnail_area.fill(draw_context, 9);
        self.draw_sprite(draw_context, 0, thumbnail_area.x, thumbnail_area.y);

        // Draw sprite sheet
        let sprite_sheet_area = Rect {
            x: 0,
            y: tools_area.bottom() + 1,
            width: 128,
            height: 34,
        };
        // TODO: Remove this, just here to make sure I'm not displaying the sprite sheet incorrectly
        sprite_sheet_area.fill(draw_context, 2);
        //

        self.draw_sprite_sheet(sprite_sheet_area.y, draw_context);

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

        color_position(self.highlighted_color).highlight(draw_context, true);

        draw_context.print(&self.bottom_text, 1, 122, 2);

        // Always render the mouse last (on top of everything)
        draw_context.raw_spr(MOUSE_SPRITE, self.mouse_x, self.mouse_y);

        print_debug_strings(draw_context, 10, 100);
    }
}

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

    pub fn highlight(&self, draw_context: &mut DrawContext, include_inner: bool) {
        let Rect {
            x,
            y,
            width,
            height,
        } = *self;

        if include_inner {
            draw_context.rect(x, y, x + width - 1, y + height - 1, 0)
        };
        draw_context.rect(x - 1, y - 1, x + width, y + height, 7);
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
