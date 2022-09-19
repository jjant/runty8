use crate::line::{horizontal_line, vertical_line};
use crate::Graphics;

/// Iterate over rectangle surface/interior.
#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Iterator over the points of the surface of a rectangle.
    /// Iteration order is unspecified.
    pub fn surface(self) -> impl Graphics {
        let width: i32 = self
            .width
            .try_into()
            .expect(&format!("Couldn't convert width {} to i32", self.width));

        let height: i32 = self
            .height
            .try_into()
            .expect(&format!("Couldn't convert height {} to i32", self.height));

        let top_bottom = [
            horizontal_line(self.x, self.x + width - 1, self.y),
            horizontal_line(self.x, self.x + width - 1, self.y + height - 1),
        ]
        .into_iter()
        .flatten();

        let left_right = [
            vertical_line(self.x, self.y + 1, self.y + height - 2),
            vertical_line(self.x + width - 1, self.y + 1, self.y + height - 2),
        ]
        .into_iter()
        .flatten();

        top_bottom.chain(left_right)
    }

    /// Iterator over the horizontal lines of the rectangle.
    /// Includes surface and interior.
    pub fn horizontal_lines(
        self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = (i32, i32)>> {
        let x0 = self.x;
        let x1 = self.x + self.width as i32 - 1;

        (self.y..self.y + self.height as i32).map(move |y| horizontal_line(x0, x1, y))
    }
}

/// Iterator over the points of a rectangle (interior and border).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_rectangle() {
        assert_eq!(Rectangle::new(34, 22, 0, 0).surface().count(), 0);
    }

    #[test]
    fn small_rectangle_count() {
        assert_eq!(Rectangle::new(8, 4, 5, 3).surface().count(), 12);
    }

    #[test]
    fn small_rectangle() {
        assert_eq!(
            Rectangle::new(8, 4, 5, 3).surface().collect::<Vec<_>>(),
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
        assert_eq!(
            Rectangle::new(34, 22, 0, 0)
                .horizontal_lines()
                .flatten()
                .count(),
            0
        );
    }

    #[test]
    fn small_filled_rectangle_count() {
        assert_eq!(
            Rectangle::new(8, 4, 2, 3)
                .horizontal_lines()
                .flatten()
                .count(),
            6
        );
    }

    #[test]
    fn small_filled_rectangle() {
        let mut filled_rectangle = Rectangle::new(8, 4, 2, 3).horizontal_lines().flatten();

        assert_eq!(filled_rectangle.next(), Some((8, 4)));
        assert_eq!(filled_rectangle.next(), Some((9, 4)));
        assert_eq!(filled_rectangle.next(), Some((8, 5)));
        assert_eq!(filled_rectangle.next(), Some((9, 5)));
        assert_eq!(filled_rectangle.next(), Some((8, 6)));
        assert_eq!(filled_rectangle.next(), Some((9, 6)));
        assert_eq!(filled_rectangle.next(), None);
    }
}
