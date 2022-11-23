use crate::ui::DrawFn;
use crate::ui::Element;
use crate::Event;
use crate::Resources;
use runty8_runtime::{App, Pico8};
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
    Tick,
}

pub(crate) struct Pico8AppCompat<A> {
    app: A,
}

impl<A: App> AppCompat for Pico8AppCompat<A> {
    type Msg = Pico8AppMsg;

    fn init(pico8: &mut Pico8) -> Self {
        Self {
            app: A::init(pico8),
        }
    }

    fn update(&mut self, _: &Self::Msg, pico8: &mut Pico8) {
        self.app.update(pico8);
    }

    fn view(&mut self, _: &mut Resources) -> Element<'_, Self::Msg> {
        DrawFn::new(|draw| self.app.draw(draw)).into()
    }

    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg> {
        if let Event::Tick { .. } = event {
            vec![Pico8AppMsg::Tick]
        } else {
            vec![]
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
