mod circle;
mod line;
mod rectangle;

pub use circle::{circle, filled_circle};
pub use line::line;
pub use rectangle::Rectangle;

/// An alias for `Iterator<Item = (i32, i32)>`.
pub trait Graphics: Iterator<Item = (i32, i32)> {}

impl<Type: Iterator<Item = (i32, i32)>> Graphics for Type {}

/// Iterator over a possibly reversed iterator.
pub struct MaybeReverse<I> {
    iter: I,
    reverse: bool,
}

/// Extension trait to conditionally reverse iterators.
pub trait ReverseIf
where
    Self: Sized,
{
    fn reverse_if(self, reverse: bool) -> MaybeReverse<Self>;
}

impl<T: DoubleEndedIterator> ReverseIf for T {
    /// Conditionally reverse an iterator.
    fn reverse_if(self, reverse: bool) -> MaybeReverse<Self> {
        MaybeReverse {
            iter: self,
            reverse,
        }
    }
}

impl<I: DoubleEndedIterator> Iterator for MaybeReverse<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reverse {
            self.iter.next_back()
        } else {
            self.iter.next()
        }
    }
}
