use runty8::{
    app,
    runtime::{
        draw_context::DrawContext,
        state::{Button, State},
    },
    App,
};

fn main() {
    app::pico8::run_app::<StressLines>();
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
    fn init() -> Self {
        Self {
            mouse: MouseState::new(64, 64),
        }
    }

    fn update(&mut self, state: &State) {
        self.mouse
            .update(state.btn(Button::Mouse), state.mouse_x, state.mouse_y);
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

    fn draw(&self, draw_context: &mut DrawContext) {
        draw_context.cls();

        // Diagonal line
        draw_context.line(0, 0, 64, 64, 7);
        // Vertical line
        draw_context.line(20, 64, 20, 127, 8);
        // Horizontal line
        draw_context.line(64, 30, 127, 30, 1);

        // Inverted inputs: Diagonal line
        draw_context.line(127, 127, 64, 64, 14);
        // Inverted inputs: vertical
        draw_context.line(40, 127, 40, 64, 11);
        // Inverted inputs: horizontal
        draw_context.line(127, 80, 64, 80, 10);

        // Single point
        draw_context.line(64, 64, 64, 64, 12);

        if let MouseState::Pressed {
            start_x,
            start_y,
            current_x,
            current_y,
        } = self.mouse
        {
            draw_context.line(start_x, start_y, current_x, current_y, 7);
        }

        let (x, y) = self.mouse.position();
        draw_context.spr(8, x - 4, y - 3);
    }
}
