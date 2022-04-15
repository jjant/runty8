pub(super) enum MouseState {
    NotPressed {
        x: i32,
        y: i32,
    },
    Pressed {
        start_x: i32,
        start_y: i32,
        current_x: i32,
        current_y: i32,
    },
}

impl MouseState {
    fn new(x: i32, y: i32) -> Self {
        MouseState::NotPressed { x, y }
    }

    fn update(&mut self, pressed: bool, x: i32, y: i32) {
        if pressed {
            let (start_x, start_y) = self.start_position();
            *self = MouseState::Pressed {
                start_x,
                start_y,
                current_x: x,
                current_y: y,
            }
        } else {
            *self = MouseState::NotPressed { x, y }
        }
    }

    fn start_position(&self) -> (i32, i32) {
        match self {
            &MouseState::NotPressed { x, y } => (x, y),
            &MouseState::Pressed {
                start_x, start_y, ..
            } => (start_x, start_y),
        }
    }
    fn position(&self) -> (i32, i32) {
        match self {
            &MouseState::NotPressed { x, y } => (x, y),
            &MouseState::Pressed {
                current_x,
                current_y,
                ..
            } => (current_x, current_y),
        }
    }
}
