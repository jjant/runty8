use runty8::ui::button::{self, Button};
use runty8::ui::text::Text;
use runty8::ui::{
    self,
    cursor::{self, Cursor},
    ElmApp2,
};
use runty8::ui::{DrawFn, Sub, Tree, Widget};

fn main() {
    ui::run_app2::<MyApp>();
}

#[derive(Debug)]
struct MyApp {
    counter: i32,
    cursor: cursor::State,
    tab: Tab,
    selected_color: u8,
    sprite_button_state: button::State,
    map_button_state: button::State,
    plus_button: button::State,
    minus_button: button::State,
    color_selector_state: [button::State; 16],
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tab {
    SpriteEditor,
    MapEditor,
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    Delta(i32),
    SpriteButtonClicked,
    MapButtonClicked,
    SelectColor(usize),
}

impl ElmApp2 for MyApp {
    type Msg = Msg;

    fn init() -> Self {
        Self {
            counter: 0,
            cursor: cursor::State::new(),
            sprite_button_state: button::State::new(),
            map_button_state: button::State::new(),
            plus_button: button::State::new(),
            minus_button: button::State::new(),
            tab: Tab::SpriteEditor,
            selected_color: 0,
            color_selector_state: [
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
                button::State::new(),
            ],
        }
    }

    fn update(&mut self, msg: &Self::Msg) {
        match msg {
            Msg::Delta(delta) => self.counter += delta,
            Msg::SpriteButtonClicked => {
                self.tab = Tab::SpriteEditor;
                println!("Sprite button clicked");
            }
            Msg::MapButtonClicked => {
                self.tab = Tab::MapEditor;
                println!("Map button clicked");
            }
            Msg::SelectColor(selected_color) => {
                self.selected_color = *selected_color as u8;
            }
        }
    }

    fn view(&mut self) -> Tree<'_, Self::Msg> {
        use Msg::*;
        let text = format!("MY APP {:?}", self.counter);

        let top_bar = DrawFn::new(|draw| {
            draw.rectfill(0, 0, 127, 7, 8);
        });

        Tree::new(vec![
            DrawFn::new(|draw| draw.cls()),
            top_bar,
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
            sprite_editor_button(&mut self.sprite_button_state, self.tab),
            map_editor_button(&mut self.map_button_state, self.tab),
            color_selector(
                70,
                50,
                12,
                self.selected_color,
                &mut self.color_selector_state,
                Msg::SelectColor,
            ),
            Cursor::new(&mut self.cursor),
        ])
    }

    fn subscriptions(&self) -> Sub<Self::Msg> {
        Sub::NoSub
    }
}

fn sprite_editor_button<'a>(
    state: &'a mut button::State,
    tab: Tab,
) -> Box<dyn Widget<Msg = Msg> + 'a> {
    let selected = tab == Tab::SpriteEditor;

    Button::new(
        110,
        0,
        8,
        8,
        Some(Msg::SpriteButtonClicked),
        state,
        DrawFn::new(move |draw| {
            let color = if selected { 2 } else { 15 };

            draw.pal(15, color);
            draw.spr(63, 0, 0);
            draw.pal(15, 15);
        }),
    )
}

fn map_editor_button<'a>(
    state: &'a mut button::State,
    tab: Tab,
) -> Box<dyn Widget<Msg = Msg> + 'a> {
    let selected = tab == Tab::MapEditor;

    Button::new(
        118,
        0,
        8,
        8,
        Some(Msg::MapButtonClicked),
        state,
        DrawFn::new(move |draw| {
            let color = if selected { 2 } else { 15 };

            draw.pal(15, color);
            draw.spr(62, 0, 0);
            draw.pal(15, 15);
        }),
    )
}

fn color_selector<'a>(
    start_x: i32,
    start_y: i32,
    tile_size: i32,
    selected_color: u8,
    states: &'a mut [button::State],
    on_press: impl (Fn(usize) -> Msg) + Copy + 'static,
) -> Box<dyn Widget<Msg = Msg> + 'a> {
    let mut v = Vec::with_capacity(16);

    let coordinates = move |index| {
        let i = index % 4;
        let j = index / 4;
        let x = start_x + i as i32 * tile_size;
        let y = start_y + j as i32 * tile_size;

        (x, y)
    };

    for (index, state) in states.iter_mut().enumerate() {
        let (x, y) = coordinates(index);

        let button: Box<dyn Widget<Msg = Msg> + 'a> = Button::new(
            x,
            y,
            tile_size,
            tile_size,
            Some(on_press(index)),
            state,
            DrawFn::new(move |draw| {
                draw.palt(None);
                draw.rectfill(0, 0, tile_size - 1, tile_size - 1, index as u8);
                draw.palt(Some(0));
            }),
        );
        v.push(button);
    }

    // Draw highlight
    v.push(DrawFn::new(move |draw| {
        let (x, y) = coordinates(selected_color as usize);

        draw.palt(None);
        draw.rect(x, y, x + tile_size - 1, y + tile_size - 1, 0);
        draw.rect(x - 1, y - 1, x + tile_size, y + tile_size, 7);
        draw.palt(Some(0));
    }));

    Box::new(Tree::new(v))
}
