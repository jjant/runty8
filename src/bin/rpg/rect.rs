use runty8::{Color, Pico8};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    // x, y: position of the top left corner
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub const fn centered(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x: x - w / 2,
            y: y - h / 2,
            w,
            h,
        }
    }

    // Top-most pixel (contained in the rect)
    pub const fn top(&self) -> i32 {
        self.y
    }

    #[allow(dead_code)]
    // Bottom-most pixel (contained in the rect)
    pub const fn bottom(&self) -> i32 {
        self.y + self.h - 1
    }

    // Left-most pixel (contained in the rect)
    pub const fn left(&self) -> i32 {
        self.x
    }

    // Right-most pixel (contained in the rect)
    pub const fn right(&self) -> i32 {
        self.x + self.w - 1
    }

    pub const fn translate(&self, x: i32, y: i32) -> Self {
        Self::new(self.x + x, self.y + y, self.w, self.h)
    }

    pub const fn intersects(&self, other: Rect) -> bool {
        // Y-axis comparisons flipped because our coordinate system
        // has Y increasing down
        // https://stackoverflow.com/a/306332/4996524
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
    }

    pub fn outline(&self, draw: &mut Pico8, color: Color) {
        if self.is_empty() {
            return;
        }
        draw.rect(
            self.x,
            self.y,
            self.x + self.w - 1,
            self.y + self.h - 1,
            color,
        )
    }

    pub fn fill(&self, draw: &mut Pico8, color: Color) {
        if self.is_empty() {
            return;
        }

        draw.rectfill(
            self.x,
            self.y,
            self.x + self.w - 1,
            self.y + self.h - 1,
            color,
        )
    }

    fn is_empty(&self) -> bool {
        self.w <= 0 || self.h <= 0
    }
}
