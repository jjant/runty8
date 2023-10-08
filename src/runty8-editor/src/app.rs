use crate::ui::DrawFn;
use crate::ui::Element;
use crate::Resources;
use runty8_core::Input;
use runty8_core::InputEvent;
use runty8_core::{App, Event, Pico8};
use std::fmt::Debug;

/// An Elm-style app
// TODO: Add link to elm/explain what this is/decide if we even want this to be public
pub trait ElmApp {
    type Msg: Copy + Debug;
    fn init() -> Self;
    fn update(&mut self, msg: &Self::Msg, resources: &mut Resources);
    fn view(&mut self, resources: &Resources) -> Element<'_, Self::Msg>;
    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg>;
}

/// Wrapper structs
pub(crate) struct ElmAppCompat<A> {
    app: A,
}

impl<A: ElmApp> AppCompat for ElmAppCompat<A> {
    type Msg = A::Msg;

    fn init(_: &mut Pico8) -> Self {
        Self { app: A::init() }
    }

    fn update(&mut self, msg: &Self::Msg, pico8: &mut Pico8) {
        self.app.update(msg, &mut pico8.resources)
    }

    fn view(&mut self, resources: &mut Resources) -> Element<'_, Self::Msg> {
        self.app.view(resources)
    }

    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg> {
        self.app.subscriptions(event)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Pico8AppMsg {
    Tick { delta_millis: f64 },
    Input(InputEvent),
}

pub(crate) struct Pico8AppCompat<A> {
    app: A,
    keys: Input,
    accumulated_delta: f64,
    delta_time: f64,
}

impl<A: App> AppCompat for Pico8AppCompat<A> {
    type Msg = Pico8AppMsg;

    fn init(pico8: &mut Pico8) -> Self {
        let fps = 30_f64;

        Self {
            app: A::init(pico8),
            keys: Input::new(),
            accumulated_delta: 0.0,
            delta_time: 1000.0 / fps,
        }
    }

    fn update(&mut self, msg: &Self::Msg, pico8: &mut Pico8) {
        match *msg {
            Pico8AppMsg::Tick { delta_millis } => {
                self.accumulated_delta += delta_millis;

                while self.accumulated_delta > self.delta_time {
                    pico8.state.update_input(&mut self.keys);
                    self.app.update(pico8);
                    self.accumulated_delta -= self.delta_time;
                }
            }

            Pico8AppMsg::Input(event) => {
                self.keys.on_event(event);
            }
        }
    }

    fn view(&mut self, _: &mut Resources) -> Element<'_, Self::Msg> {
        DrawFn::new(|draw| self.app.draw(draw)).into()
    }

    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg> {
        match *event {
            Event::Input(input_event) => vec![Pico8AppMsg::Input(input_event)],
            Event::Tick { delta_millis } => vec![Pico8AppMsg::Tick { delta_millis }],
            Event::WindowClosed => vec![],
        }
    }
}

/// Not intended for direct use.
pub(crate) trait AppCompat {
    type Msg: Copy + Debug;
    fn init(pico8: &mut Pico8) -> Self;
    fn update(&mut self, msg: &Self::Msg, pico8: &mut Pico8);
    fn view(&mut self, resources: &mut Resources) -> Element<'_, Self::Msg>;
    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg>;
}
