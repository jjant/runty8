use runty8::ui::button::{self, Button};
use runty8::ui::text::Text;
use runty8::ui::{
    self,
    cursor::{self, Cursor},
    ElmApp2,
};
use runty8::ui::{DrawFn, Sub, Tree};

fn main() {
    ui::run_app2::<MyApp>();
}

#[derive(Debug)]
struct MyApp {
    counter: i32,
    cursor: cursor::State,
    sprite_button_state: button::State,
    plus_button: button::State,
    minus_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    Delta(i32),
    SpriteButton,
}

impl ElmApp2 for MyApp {
    type Msg = Msg;

    fn init() -> Self {
        Self {
            counter: 0,
            cursor: cursor::State::new(),
            sprite_button_state: button::State::new(),
            plus_button: button::State::new(),
            minus_button: button::State::new(),
        }
    }

    fn update(&mut self, msg: &Self::Msg) {
        match msg {
            Msg::Delta(delta) => self.counter += delta,
            Msg::SpriteButton => {
                println!("Sprite button clicked");
            }
        }
    }

    fn view(&mut self) -> Tree<'_, Self::Msg> {
        use Msg::*;
        let text = format!("MY APP {:?}", self.counter);

        let editor_icon = Button::new(
            110,
            0,
            8,
            8,
            Some(SpriteButton),
            &mut self.sprite_button_state,
            DrawFn::new(|draw| draw.spr(63, 0, 0)),
        );
        let map_icon = DrawFn::new(|draw| draw.spr(62, 118, 0));

        Tree::new(vec![
            DrawFn::new(|draw| draw.cls()),
            Button::new(
                56,
                32,
                12,
                12,
                Some(Delta(1)),
                &mut self.plus_button,
                Text::new("+1".to_string(), 0, 0, 7),
            ),
            Button::new(
                56,
                64,
                12,
                12,
                Some(Delta(-1)),
                &mut self.minus_button,
                Text::new("-1".to_string(), 0, 0, 7),
            ),
            Text::new(text, 0, 60, 7),
            editor_icon,
            map_icon,
            Cursor::new(&mut self.cursor),
        ])
    }

    fn subscriptions(&self) -> Sub<Self::Msg> {
        Sub::NoSub
    }
}
