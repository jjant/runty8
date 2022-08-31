use crate::line::{horizontal_line, vertical_line};
use crate::Graphics;

pub struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

pub struct MaybeReverse<I> {
    iter: I,
    reverse: bool,
}

pub trait ReverseIf
where
    Self: Sized,
{
    fn reverse_if(self, reverse: bool) -> MaybeReverse<Self>;
}

impl<T: DoubleEndedIterator> ReverseIf for T {
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

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn horizontal_lines(
        self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = (i32, i32)>> {
        (self.y..self.y + self.height)
            .map(move |y| horizontal_line(self.x, self.x + self.width - 1, y))
    }
}

/// Iterator over the points of the border of a rectangle.
pub fn rectangle(x: i32, y: i32, width: u32, height: u32) -> impl Graphics {
    let width: i32 = width
        .try_into()
        .expect(&format!("Couldn't convert width {} to i32", width));

    let height: i32 = height
        .try_into()
        .expect(&format!("Couldn't convert height {} to i32", height));

    let top_bottom = [
        horizontal_line(x, x + width - 1, y),
        horizontal_line(x, x + width - 1, y + height - 1),
    ]
    .into_iter()
    .flatten();

    let left_right = [
        vertical_line(x, y + 1, y + height - 2),
        vertical_line(x + width - 1, y + 1, y + height - 2),
    ]
    .into_iter()
    .flatten();

    top_bottom.chain(left_right)
}

/// Iterator over the points of a rectangle (interior and border).
pub fn filled_rectangle(x: i32, y: i32, width: u32, height: u32) -> impl Graphics {
    let width: i32 = width
        .try_into()
        .expect(&format!("Couldn't convert width {} to i32", width));

    let height: i32 = height
        .try_into()
        .expect(&format!("Couldn't convert height {} to i32", height));

    (y..y + height).flat_map(move |y| {
        // Horizontal line includes endpoints
        let x1 = x + width - 1;
        horizontal_line(x, x1, y)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_rectangle() {
        assert_eq!(rectangle(34, 22, 0, 0).count(), 0);
    }

    #[test]
    fn small_rectangle_count() {
        assert_eq!(rectangle(8, 4, 5, 3).count(), 12);
    }

    #[test]
    fn small_rectangle() {
        assert_eq!(
            rectangle(8, 4, 5, 3).collect::<Vec<_>>(),
            vec![
                (8, 4), // Top
                (9, 4),
                (10, 4),
                (11, 4),
                (12, 4),
                (8, 6), // Bottom
                (9, 6),
                (10, 6),
                (11, 6),
                (12, 6),
                (8, 5),  // Left
                (12, 5), // Rigth
            ]
        );
    }

    #[test]
    fn empty_filled_rectangle() {
        assert_eq!(filled_rectangle(34, 22, 0, 0).count(), 0);
    }

    #[test]
    fn small_filled_rectangle_count() {
        assert_eq!(filled_rectangle(8, 4, 2, 3).count(), 6);
    }

    #[test]
    fn small_filled_rectangle() {
        let mut filled_rectangle = filled_rectangle(8, 4, 2, 3);

        assert_eq!(filled_rectangle.next(), Some((8, 4)));
        assert_eq!(filled_rectangle.next(), Some((9, 4)));
        assert_eq!(filled_rectangle.next(), Some((8, 5)));
        assert_eq!(filled_rectangle.next(), Some((9, 5)));
        assert_eq!(filled_rectangle.next(), Some((8, 6)));
        assert_eq!(filled_rectangle.next(), Some((9, 6)));
        assert_eq!(filled_rectangle.next(), None);
    }
}
