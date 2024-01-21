use std::ops::{Add, Div, Mul, Neg, Sub};

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

impl Mul<Vec2<i32>> for i32 {
    type Output = Vec2<i32>;

    fn mul(self, rhs: Vec2<i32>) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
impl Div<i32> for Vec2<i32> {
    type Output = Vec2<i32>;

    fn div(self, rhs: i32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

pub(crate) type Vec2i = Vec2<i32>;

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Convert this vec's elements into another type.
    #[allow(unused)]
    pub fn convert<U: From<T>>(self) -> Vec2<U> {
        Vec2::new(self.x.into(), self.y.into())
    }

    /// Try to convert this vec's elements into another type.
    pub fn try_convert<U: TryFrom<T>>(self) -> Result<Vec2<U>, U::Error> {
        let x = self.x.try_into()?;
        let y = self.y.try_into()?;

        Ok(Vec2::new(x, y))
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

    #[test]
    fn mul_works() {
        assert_eq!(0 * vec2(23148, 31241), vec2(0, 0));
        assert_eq!(2 * vec2(0, 3), vec2(0, 6));
        assert_eq!(3 * vec2(12, 42), vec2(36, 126));
    }

    #[test]
    fn div_works() {
        assert_eq!(vec2(0, 3) / 2, vec2(0, 1));
        assert_eq!(vec2(12, 42) / 3, vec2(4, 14));
    }
}
