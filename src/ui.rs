use crate::{DrawContext, Event, MouseButton, MouseEvent};
use enum_dispatch::enum_dispatch;

pub struct Button<Msg> {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    on_press: Option<Msg>,
    state: State,
}

struct State {
    pressed: bool,
}

impl<Msg> Button<Msg> {
    fn new(x: i32, y: i32, width: i32, height: i32, on_press: Option<Msg>) -> Self {
        Button {
            x,
            y,
            width,
            height,
            on_press,
            state: State { pressed: false },
        }
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        let contains_x = x >= self.x && x < self.width;
        let contains_y = y >= self.y && y < self.height;

        contains_x && contains_y
    }
}

#[enum_dispatch]
pub trait Widget {
    type Msg: Copy;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut impl FnMut(Self::Msg),
    );

    fn draw(&self, draw: &mut DrawContext);
}

impl<Msg: Copy> Widget for Button<Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut impl FnMut(Self::Msg),
    ) {
        use Event::*;
        use MouseEvent::*;

        match event {
            Mouse(Down(MouseButton::Left)) => {
                if self.contains(cursor_position.0, cursor_position.1) {
                    self.state.pressed = true;
                }
            }
            Mouse(Up(MouseButton::Left)) => {
                if self.contains(cursor_position.0, cursor_position.1) && self.state.pressed {
                    if let Some(on_press) = self.on_press {
                        dispatch_event(on_press);
                    }
                }

                self.state.pressed = false;
            }
            _ => {}
        }
    }

    fn draw(&self, draw: &mut DrawContext) {
        let color = if self.state.pressed { 5 } else { 9 };

        // TODO: Handle properly
        draw.rectfill(
            self.x,
            self.y,
            self.x + self.width - 1,
            self.y + self.height - 1,
            color,
        );
    }
}

impl<T: Widget> Widget for Vec<T> {
    type Msg = T::Msg;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut impl FnMut(Self::Msg),
    ) {
        for w in self.iter_mut() {
            w.on_event(event, cursor_position, dispatch_event);
        }
    }

    fn draw(&self, draw: &mut DrawContext) {
        for w in self.iter() {
            w.draw(draw);
        }
    }
}

pub enum WidgetImpl<Msg> {
    Tree(Vec<WidgetImpl<Msg>>),
    Button(Button<Msg>),
}

impl<Msg: Copy> Widget for WidgetImpl<Msg> {
    type Msg = Msg;

    fn on_event(
        &mut self,
        event: Event,
        cursor_position: (i32, i32),
        dispatch_event: &mut impl FnMut(Self::Msg),
    ) {
        match self {
            WidgetImpl::Tree(x) => x.on_event(event, cursor_position, dispatch_event),
            WidgetImpl::Button(x) => x.on_event(event, cursor_position, dispatch_event),
        }
    }

    fn draw(&self, draw: &mut DrawContext) {
        match self {
            WidgetImpl::Tree(x) => x.draw(draw),
            WidgetImpl::Button(x) => x.draw(draw),
        }
    }
}

pub trait ElmApp2 {
    type Msg: Copy;

    fn init() -> Self;

    fn update(&mut self, msg: &Self::Msg);

    fn view(&self) -> WidgetImpl<Self::Msg>;
}

struct MyApp {
    counter: i32,
}

impl ElmApp2 for MyApp {
    type Msg = i32;

    fn init() -> Self {
        Self { counter: 0 }
    }

    fn update(&mut self, delta: &Self::Msg) {
        self.counter += delta;
    }

    fn view(&self) -> WidgetImpl<Self::Msg> {
        WidgetImpl::Tree(vec![
            WidgetImpl::Button(Button::new(0, 0, 16, 16, Some(1))),
            WidgetImpl::Button(Button::new(0, 32, 16, 16, Some(-1))),
        ])
    }
}

fn run_app<T: ElmApp2>() {
    let mut app = T::init();

    let mut msg_queue = vec![];

    let mut draw_context = DrawContext::new(unsafe { std::mem::zeroed() });
    loop {
        let widget = app.view();
        widget.draw(&mut draw_context);

        let event = todo!();
        let cursor_position = (0, 0);
        let dispatch_event = |msg| msg_queue.push(msg);

        widget.on_event(event, cursor_position, &mut dispatch_event);

        for msg in msg_queue.into_iter() {
            app.update(&msg);
        }
    }
}
