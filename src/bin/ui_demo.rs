use runty8::ui::button::{self, Button};
use runty8::ui::text::Text;
use runty8::ui::DrawFn;
use runty8::ui::{
    self,
    cursor::{self, Cursor},
    ElmApp2, WidgetImpl,
};

fn main() {
    ui::run_app2::<MyApp>();
}

#[derive(Debug)]
struct MyApp {
    counter: i32,
    cursor: cursor::State,
    plus_button: button::State,
    minus_button: button::State,
}

impl ElmApp2 for MyApp {
    type Msg = i32;

    fn init() -> Self {
        Self {
            counter: 0,
            cursor: cursor::State::new(),
            plus_button: button::State::new(),
            minus_button: button::State::new(),
        }
    }

    fn update(&mut self, delta: &Self::Msg) {
        self.counter += delta;
    }

    fn view(&mut self) -> WidgetImpl<Self::Msg> {
        let text = format!("MY APP {:?}", self.counter);

        WidgetImpl::Tree(vec![
            WidgetImpl::DrawFn(DrawFn::new(|draw| draw.cls())),
            WidgetImpl::Button(Button::new(
                56,
                32,
                12,
                12,
                Some(1),
                &mut self.plus_button,
                Box::new(WidgetImpl::Tree(vec![])),
            )),
            WidgetImpl::Button(Button::new(
                56,
                64,
                12,
                12,
                Some(-1),
                &mut self.minus_button,
                Box::new(Text::new("HI".to_string(), 56, 64, 7)),
            )),
            WidgetImpl::Text(Text::new(text, 0, 60, 7)),
            WidgetImpl::Cursor(Cursor::new(&mut self.cursor)),
        ])
    }
}
