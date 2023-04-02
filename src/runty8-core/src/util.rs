#[derive(Debug)]
pub(crate) struct MinMax<T> {
    pub(crate) min: T,
    pub(crate) max: T,
}

pub(crate) fn min_max<T: Ord>(a: T, b: T) -> MinMax<T> {
    if a < b {
        MinMax { min: a, max: b }
    } else {
        MinMax { min: b, max: a }
    }
}

#[cfg(test)]
mod tests {
    use super::{min_max, MinMax};

    #[test]
    fn min_max_works() {
        assert!(matches!(min_max(2, 3), MinMax { min: 2, max: 3 }));
        assert!(matches!(min_max(3, 2), MinMax { min: 2, max: 3 }));
        assert!(matches!(min_max(0, 0), MinMax { min: 0, max: 0 }));
    }
}
