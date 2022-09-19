use crate::Graphics;

/// Iterator over the points of a line (includes endpoints).
pub fn line(x0: i32, y0: i32, x1: i32, y1: i32) -> impl Graphics {
    LineIter::new(x0, y0, x1, y1)
}

struct LineIter {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    dx: i32,
    dy: i32,
    sx: i32,
    sy: i32,
    err: i32,
    should_exit: bool,
}

impl LineIter {
    fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let err = dx + dy;

        Self {
            x0,
            y0,
            x1,
            y1,
            dx,
            dy,
            sx,
            sy,
            err,
            should_exit: false,
        }
    }
}

impl Iterator for LineIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.should_exit {
            return None;
        }
        // Uses Bresenham's algorithm, last snippet in this article
        // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm

        let ret = (self.x0, self.y0);
        if self.x0 == self.x1 && self.y0 == self.y1 {
            self.should_exit = true;
        }

        let e2 = 2 * self.err;
        if e2 >= self.dy {
            /* e_xy+e_x > 0 */
            self.err += self.dy;
            self.x0 += self.sx;
        }
        if e2 <= self.dx {
            /* e_xy+e_y < 0 */
            self.err += self.dx;
            self.y0 += self.sy;
        }

        Some(ret)
    }
}

pub fn horizontal_line(x0: i32, x1: i32, y: i32) -> impl DoubleEndedIterator<Item = (i32, i32)> {
    (x0..=x1).map(move |x| (x, y))
}

pub fn vertical_line(x: i32, y0: i32, y1: i32) -> impl Graphics {
    (y0..=y1).map(move |y| (x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_is_horizontal() {
        assert_eq!(
            line(20, 0, 30, 0).collect::<Vec<_>>(),
            horizontal_line(20, 30, 0).collect::<Vec<_>>()
        );
    }

    #[test]
    fn line_is_vertical() {
        assert_eq!(
            line(0, 20, 0, 30).collect::<Vec<_>>(),
            vertical_line(0, 20, 30).collect::<Vec<_>>()
        );
    }

    #[test]
    fn line_is_diagonal() {
        assert_eq!(
            line(2, 2, 5, 5).collect::<Vec<_>>(),
            vec![(2, 2), (3, 3), (4, 4), (5, 5)]
        )
    }
    // TODO: Test cases with inverted params (x1 > x0, etc)
}
