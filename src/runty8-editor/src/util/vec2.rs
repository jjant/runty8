use std::ops::{Add, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl Vec2i {
    pub(crate) fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl<T: Add> Add for Vec2<T> {
    type Output = Vec2<T::Output>;

    fn add(self, other: Self) -> Self::Output {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub> Sub for Vec2<T> {
    type Output = Vec2<T::Output>;

    fn sub(self, other: Self) -> Self::Output {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Neg> Neg for Vec2<T> {
    type Output = Vec2<T::Output>;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub(crate) type Vec2i = Vec2<i32>;

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub(crate) fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_works() {
        assert_eq!(vec2(1, 2) + vec2(2, 3), vec2(3, 5));
        assert_eq!(vec2(4, 1234) + vec2(0, 0), vec2(4, 1234));
    }

    #[test]
    fn negation_works() {
        assert_eq!(-vec2(1, 2), vec2(-1, -2));
        assert_eq!(-vec2(4, 1234), vec2(-4, -1234));
    }
}
