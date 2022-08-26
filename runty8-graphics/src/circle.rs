use crate::Graphics;

struct NorthwestOctant {
    x: i32,
    y: i32,
    d: i32,
}

impl NorthwestOctant {
    fn new(r: i32) -> Self {
        Self {
            x: 0,
            y: r,
            d: 3 - 2 * r,
        }
    }
}

impl Iterator for NorthwestOctant {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.x {
            return None;
        }
        let ret_value = (self.x, self.y);

        let pk = self.d;
        self.x += 1;
        if self.d < 0 {
            self.d += 4 * self.x + 6;
        } else {
            self.y -= 1;
            self.d += 4 * (self.x - self.y) + 10;
        }

        Some(ret_value)
    }
}

fn northwest_octant(r: u32) -> impl Graphics {
    NorthwestOctant::new(
        r.try_into()
            .expect(&format!("Couldn't convert radius {} to i32", r)),
    )
}

/// Iterator for points in a circle
pub fn circle(cx: i32, cy: i32, r: u32) -> impl Graphics {
    northwest_octant(r)
        .flat_map(|(x, y)| {
            [
                (x, y),
                (x, -y),
                (-x, y),
                (-x, -y),
                (y, x),
                (y, -x),
                (-y, x),
                (-y, -x),
            ]
            .into_iter()
        })
        .map(move |(x, y)| (cx + x, cy + y))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Examples from <https://www.gatevidyalay.com/bresenham-circle-drawing-algorithm/>
    #[test]
    fn radius_0_northwest_octant() {
        assert_eq!(northwest_octant(0).collect::<Vec<_>>(), vec![(0, 0)]);
    }

    #[test]
    fn radius_8_northwest_octant() {
        assert_eq!(
            northwest_octant(8).collect::<Vec<_>>(),
            vec![(0, 8), (1, 8), (2, 8), (3, 7), (4, 6), (5, 5)]
        );
    }

    #[test]
    fn radius_10_northwest_octant() {
        assert_eq!(
            northwest_octant(10).collect::<Vec<_>>(),
            vec![(0, 10), (1, 10), (2, 10), (3, 9), (4, 9), (5, 8), (6, 7)]
        );
    }
}
