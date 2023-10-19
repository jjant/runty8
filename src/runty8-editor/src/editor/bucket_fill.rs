pub struct PixelsMut<'a, T> {
    buffer: &'a mut [T],
    width: usize,
    height: usize,
}

impl<'a, T> PixelsMut<'a, T> {
    pub fn new(buffer: &'a mut [T], width: usize) -> Self {
        let height = buffer.len() / width;
        if height * width != buffer.len() {
            panic!("Buffer is not square");
        }

        Self {
            buffer,
            width,
            height,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<&T> {
        self.buffer.get(self.index(x, y)?)
    }

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        self.buffer.get_mut(self.index(x, y)?)
    }

    // TODO: Handle correct indexing
    fn index(&self, x: isize, y: isize) -> Option<usize> {
        let x: usize = x.try_into().ok()?;
        let x = if x < self.width { Some(x) } else { None }?;

        let y: usize = y.try_into().ok()?;
        let y = if y < self.height { Some(y) } else { None }?;

        Some(x + y * self.width)
    }
}

impl<'a, T: PartialEq + Copy> PixelsMut<'a, T> {
    pub fn fill_bucket(&mut self, color: T, clicked_x: isize, clicked_y: isize) {
        let color_at_clicked_position: T = match self.get(clicked_x, clicked_y).copied() {
            Some(color) => color,
            // Click was outside the buffer
            None => return,
        };

        if color_at_clicked_position == color {
            return;
        }

        let mut queue = vec![(clicked_x, clicked_y)];

        let mut i = 0;
        while i < queue.len() {
            let current_position = queue[i];
            let current_color = self
                .get_mut(current_position.0, current_position.1)
                .unwrap();

            // Paint current and check neighbors
            *current_color = color;
            for (dir_x, dir_y) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                let neighbor_position = (current_position.0 + dir_x, current_position.1 + dir_y);
                let neighboring_color = self.get(neighbor_position.0, neighbor_position.1).copied();

                if neighboring_color == Some(color_at_clicked_position) {
                    queue.push(neighbor_position);
                }
            }

            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PixelsMut;

    #[test]
    fn paints_already_painted_buffer() {
        let mut pixels = [
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
        ];
        let mut pixels_mut = PixelsMut::new(&mut pixels, 8);

        pixels_mut.fill_bucket(0, 3, 3);
        assert_eq!(
            pixels,
            [
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, //
            ]
        )
    }

    #[test]
    fn paints_whole_buffer() {
        let mut pixels = [
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 7, //
            0, 0, 0, 0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, 0, 0, 0, //
        ];
        let mut pixels_mut = PixelsMut::new(&mut pixels, 8);

        pixels_mut.fill_bucket(7, 3, 3);
        assert_eq!(
            pixels,
            [
                7, 7, 7, 7, 7, 7, 7, 7, //
                7, 7, 7, 7, 7, 7, 7, 7, //
                7, 7, 7, 7, 7, 7, 7, 7, //
                7, 7, 7, 7, 7, 7, 7, 7, //
                7, 7, 7, 7, 7, 7, 7, 7, //
                7, 7, 7, 7, 7, 7, 7, 7, //
                7, 7, 7, 7, 7, 7, 7, 7, //
                7, 7, 7, 7, 7, 7, 7, 7, //
            ]
        )
    }

    #[test]
    fn example() {
        let mut pixels = [
            1, 1, 1, 1, 1, 0, 0, //
            1, 1, 1, 0, 1, 1, 1, //
            0, 0, 0, 0, 0, 0, 1, //
            0, 0, 0, 0, 0, 0, 1, //
            1, 1, 0, 0, 0, 0, 1, //
            1, 1, 0, 0, 0, 0, 1, //
            1, 1, 1, 1, 1, 1, 1, //
        ];
        let mut pixels_mut = PixelsMut::new(&mut pixels, 7);

        pixels_mut.fill_bucket(2, 3, 3);
        assert_eq!(
            pixels,
            [
                1, 1, 1, 1, 1, 0, 0, //
                1, 1, 1, 2, 1, 1, 1, //
                2, 2, 2, 2, 2, 2, 1, //
                2, 2, 2, 2, 2, 2, 1, //
                1, 1, 2, 2, 2, 2, 1, //
                1, 1, 2, 2, 2, 2, 1, //
                1, 1, 1, 1, 1, 1, 1, //
            ]
        )
    }

    #[test]
    fn example2() {
        let mut pixels = [
            1, 0, 1, 1, 1, 1, //
            1, 0, 1, 0, 0, 0, //
            1, 1, 1, 1, 1, 1, //
            0, 0, 0, 1, 0, 0, //
            0, 0, 0, 1, 0, 0,
        ];
        let mut pixels_mut = PixelsMut::new(&mut pixels, 6);

        pixels_mut.fill_bucket(2, 5, 0);
        drop(pixels_mut);
        assert_eq!(
            pixels,
            [
                2, 0, 2, 2, 2, 2, //
                2, 0, 2, 0, 0, 0, //
                2, 2, 2, 2, 2, 2, //
                0, 0, 0, 2, 0, 0, //
                0, 0, 0, 2, 0, 0,
            ]
        );

        let mut pixels_mut = PixelsMut::new(&mut pixels, 6);
        pixels_mut.fill_bucket(3, 1, 0);
        assert_eq!(
            pixels,
            [
                2, 3, 2, 2, 2, 2, //
                2, 3, 2, 0, 0, 0, //
                2, 2, 2, 2, 2, 2, //
                0, 0, 0, 2, 0, 0, //
                0, 0, 0, 2, 0, 0,
            ]
        );
    }

    #[test]
    fn clicking_outside_the_buffer_doesnt_modify_it() {
        let mut pixels = [
            0, 1, 2, 3, //
            4, 5, 6, 7, //
        ];

        // x inside and y outside
        let mut pixels_mut = PixelsMut::new(&mut pixels, 4);
        pixels_mut.fill_bucket(99, 3, 3);
        assert_eq!(
            pixels,
            [
                0, 1, 2, 3, //
                4, 5, 6, 7, //
            ]
        );
        // x outside and y inside
        let mut pixels_mut = PixelsMut::new(&mut pixels, 4);
        pixels_mut.fill_bucket(99, -1, 2);
        assert_eq!(
            pixels,
            [
                0, 1, 2, 3, //
                4, 5, 6, 7, //
            ]
        );
        // x and y both outside
        let mut pixels_mut = PixelsMut::new(&mut pixels, 4);
        pixels_mut.fill_bucket(99, 5, 3);
        assert_eq!(
            pixels,
            [
                0, 1, 2, 3, //
                4, 5, 6, 7, //
            ]
        );
    }
}
