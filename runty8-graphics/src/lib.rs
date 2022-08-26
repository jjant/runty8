pub fn rect(x: i32, y: i32, width: u32, height: u32) -> impl Iterator<Item = (i32, i32)> {
    let width: i32 = width
        .try_into()
        .expect(&format!("Couldn't convert width {} to i32", width));

    let height: i32 = height
        .try_into()
        .expect(&format!("Couldn't convert height {} to i32", height));

    (y..y + height).flat_map(move |y| (x..x + width).map(move |x| (x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_rectangle() {
        assert_eq!(rect(34, 22, 0, 0).count(), 0);
    }

    #[test]
    fn small_rectangle_count() {
        assert_eq!(rect(8, 4, 2, 3).count(), 6);
    }

    #[test]
    fn small_rectangle() {
        let mut rectangle = rect(8, 4, 2, 3);

        assert_eq!(rectangle.next(), Some((8, 4)));
        assert_eq!(rectangle.next(), Some((9, 4)));
        assert_eq!(rectangle.next(), Some((8, 5)));
        assert_eq!(rectangle.next(), Some((9, 5)));
        assert_eq!(rectangle.next(), Some((8, 6)));
        assert_eq!(rectangle.next(), Some((9, 6)));
        assert_eq!(rectangle.next(), None);
    }
}
