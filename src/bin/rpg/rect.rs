use runty8::runtime::{draw_context::DrawContext, sprite_sheet::Color};

pub struct Rect {
    // x, y: position of the top left corner
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub fn centered(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x: x - w / 2,
            y: y - h / 2,
            w,
            h,
        }
    }

    // Top-most pixel (contained in the rect)
    pub fn top(&self) -> i32 {
        self.y
    }

    #[allow(dead_code)]
    // Bottom-most pixel (contained in the rect)
    pub fn bottom(&self) -> i32 {
        self.y + self.h - 1
    }

    // Left-most pixel (contained in the rect)
    pub fn left(&self) -> i32 {
        self.x
    }

    // Right-most pixel (contained in the rect)
    pub fn right(&self) -> i32 {
        self.x + self.w - 1
    }

    pub fn outline(&self, draw: &mut DrawContext, color: Color) {
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

    pub fn fill(&self, draw: &mut DrawContext, color: Color) {
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
