use runty8::ui::{
    self,
    cursor::{self, Cursor},
    Button, ButtonState, ElmApp2, WidgetImpl,
};

fn main() {
    ui::run_app2::<MyApp>();
}

#[derive(Debug)]
struct MyApp {
    counter: i32,
    cursor: cursor::State,
    plus_button: ButtonState,
    minus_button: ButtonState,
}

impl ElmApp2 for MyApp {
    type Msg = i32;

    fn init() -> Self {
        Self {
            counter: 0,
            cursor: cursor::State::new(),
            plus_button: ButtonState::new(),
            minus_button: ButtonState::new(),
        }
    }

    fn update(&mut self, delta: &Self::Msg) {
        self.counter += delta;
        dbg!(self);
    }

    fn view(&mut self) -> WidgetImpl<Self::Msg> {
        WidgetImpl::Tree(vec![
            WidgetImpl::Cursor(Cursor::new(&mut self.cursor)),
            WidgetImpl::Button(Button::new(0, 0, 16, 16, Some(1), &mut self.plus_button)),
            WidgetImpl::Button(Button::new(0, 32, 16, 16, Some(-1), &mut self.minus_button)),
        ])
    }
}
