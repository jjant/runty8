use runty8::{App, Button, Pico8};

fn main() {
    runty8::run_app::<StressLines>("examples/stress_lines".to_owned()).unwrap();
}
struct StressLines {
    mouse: MouseState,
}

enum MouseState {
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
        match *self {
            MouseState::NotPressed { x, y } => (x, y),
            MouseState::Pressed {
                start_x, start_y, ..
            } => (start_x, start_y),
        }
    }
    fn position(&self) -> (i32, i32) {
        match *self {
            MouseState::NotPressed { x, y } => (x, y),
            MouseState::Pressed {
                current_x,
                current_y,
                ..
            } => (current_x, current_y),
        }
    }
}

impl App for StressLines {
    fn init(_: &mut Pico8) -> Self {
        Self {
            mouse: MouseState::new(64, 64),
        }
    }

    fn update(&mut self, state: &mut Pico8) {
        let (mouse_x, mouse_y) = state.mouse();
        self.mouse
            .update(state.btn(Button::Mouse), mouse_x, mouse_y);
        // let mut i = 0;
        // while i < self.particles.len() {
        //     if self.particles[i].ttl == 0 {
        //         self.particles.remove(i);
        //     } else {
        //         self.particles[i].update();

        //         i += 1
        //     }
        // }
    }

    fn draw(&mut self, pico8: &mut Pico8) {
        pico8.cls(0);

        // Diagonal line
        pico8.line(0, 0, 64, 64, 7);
        // Vertical line
        pico8.line(20, 64, 20, 127, 8);
        // Horizontal line
        pico8.line(64, 30, 127, 30, 1);

        // Inverted inputs: Diagonal line
        pico8.line(127, 127, 64, 64, 14);
        // Inverted inputs: vertical
        pico8.line(40, 127, 40, 64, 11);
        // Inverted inputs: horizontal
        pico8.line(127, 80, 64, 80, 10);

        // Single point
        pico8.line(64, 64, 64, 64, 12);

        if let MouseState::Pressed {
            start_x,
            start_y,
            current_x,
            current_y,
        } = self.mouse
        {
            pico8.line(start_x, start_y, current_x, current_y, 7);
        }

        let (x, y) = self.mouse.position();
        pico8.spr(8, x - 4, y - 3);
    }
}
