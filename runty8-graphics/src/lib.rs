mod circle;
mod line;
mod rectangle;

pub use circle::{circle, filled_circle};
pub use line::line;
pub use rectangle::Rectangle;
pub use rectangle::ReverseIf;
pub use rectangle::{filled_rectangle, rectangle};

/// An alias for `Iterator<Item = (i32, i32)>`.
pub trait Graphics: Iterator<Item = (i32, i32)> {}

impl<Type: Iterator<Item = (i32, i32)>> Graphics for Type {}
