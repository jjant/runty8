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
            d: 1 - r,
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

        self.x += 1;
        if self.d < 0 {
            self.d += 2 * self.x + 1;
        } else {
            self.y -= 1;
            self.d += 2 * (self.x - self.y) + 1;
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

pub fn filled_circle(cx: i32, cy: i32, r: u32) -> impl Graphics {
    northwest_octant(r)
        .flat_map(|(x, y)| [(x, y), (y, x)]) // Generate a quadrant, instead of an octant
        .flat_map(|(x, y)| {
            crate::horizontal_line(-x, x, y).chain(crate::horizontal_line(-x, x, -y))
        })
        .map(move |(x, y)| (cx + x, cy + y))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Examples compiled from looking at Pico8's circ output.
    #[test]
    fn radius_0_northwest_octant() {
        assert_eq!(northwest_octant(0).collect::<Vec<_>>(), vec![(0, 0)]);
    }

    #[test]
    fn radius_1_northwest_octant() {
        assert_eq!(northwest_octant(1).collect::<Vec<_>>(), vec![(0, 1)]);
    }

    #[test]
    fn radius_2_northwest_octant() {
        assert_eq!(
            northwest_octant(2).collect::<Vec<_>>(),
            vec![(0, 2), (1, 2)]
        );
    }

    #[test]
    fn radius_3_northwest_octant() {
        assert_eq!(
            northwest_octant(3).collect::<Vec<_>>(),
            vec![(0, 3), (1, 3), (2, 2)]
        );
    }

    #[test]
    fn radius_4_northwest_octant() {
        assert_eq!(
            northwest_octant(4).collect::<Vec<_>>(),
            vec![(0, 4), (1, 4), (2, 3), (3, 3)]
        );
    }
}
