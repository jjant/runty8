mod circle;
mod rectangle;
pub use circle::{circle, filled_circle};
pub use rectangle::{filled_rectangle, rectangle};

pub trait Graphics: Iterator<Item = (i32, i32)> {}

impl<Type: Iterator<Item = (i32, i32)>> Graphics for Type {}

fn horizontal_line(x0: i32, x1: i32, y: i32) -> impl Graphics {
    (x0..=x1).map(move |x| (x, y))
}

#[allow(dead_code)]
fn vertical_line(x: i32, y0: i32, y1: i32) -> impl Graphics {
    (y0..=y1).map(move |y| (x, y))
}
