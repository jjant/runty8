mod circle;

pub trait Graphics: Iterator<Item = (i32, i32)> {}

impl<Type: Iterator<Item = (i32, i32)>> Graphics for Type {}

fn horizontal_line(x0: i32, x1: i32, y: i32) -> impl Graphics {
    (x0..x1).map(move |x| (x, y))
}

pub fn filled_rectangle(x: i32, y: i32, width: u32, height: u32) -> impl Graphics {
    let width: i32 = width
        .try_into()
        .expect(&format!("Couldn't convert width {} to i32", width));

    let height: i32 = height
        .try_into()
        .expect(&format!("Couldn't convert height {} to i32", height));

    (y..y + height).flat_map(move |y| horizontal_line(x, x + width, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_rectangle() {
        assert_eq!(filled_rectangle(34, 22, 0, 0).count(), 0);
    }

    #[test]
    fn small_rectangle_count() {
        assert_eq!(filled_rectangle(8, 4, 2, 3).count(), 6);
    }

    #[test]
    fn small_rectangle() {
        let mut rectangle = filled_rectangle(8, 4, 2, 3);

        assert_eq!(rectangle.next(), Some((8, 4)));
        assert_eq!(rectangle.next(), Some((9, 4)));
        assert_eq!(rectangle.next(), Some((8, 5)));
        assert_eq!(rectangle.next(), Some((9, 5)));
        assert_eq!(rectangle.next(), Some((8, 6)));
        assert_eq!(rectangle.next(), Some((9, 6)));
        assert_eq!(rectangle.next(), None);
    }
}
