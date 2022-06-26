use crate::runtime::{
    draw_context::{DrawContext, DrawData},
    state::{InternalState, State},
};
use crate::ui::{Element, Tree};
use crate::Event;
use crate::Resources;
use std::fmt::Debug;

/// A regular pico8 app
pub trait App: WhichOne<Which = Left> {
    fn init(state: &State) -> Self;
    fn update(&mut self, state: &State);
    fn draw(&mut self, draw_context: &mut DrawContext);
}

pub trait ElmApp: WhichOne<Which = Right> {
    type Msg: Copy + Debug;
    fn init() -> Self;
    fn update(&mut self, msg: &Self::Msg, resources: &mut Resources);
    fn view(&mut self, resources: &Resources) -> Element<'_, Self::Msg>;
    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg>;
}

///////////////////////////////////////////////////////////////////////////////
//      Declaring the mutually exclusive LeftTrait/RightTrait traits
///////////////////////////////////////////////////////////////////////////////

pub trait WhichOne {
    type Which;
}

pub struct Left;
pub struct Right;

trait LeftTrait: WhichOne<Which = Left> {}

trait RightTrait: WhichOne<Which = Right> {}

///////////////////////////////////////////////////////////////////////////////
// Implementing a trait differently based on whether a type implements
// LeftTrait or RightTrait
///////////////////////////////////////////////////////////////////////////////

/// Not intended for direct use.
pub trait AppCompat {
    type Msg: Copy + Debug;
    fn init(state: &State) -> Self;
    fn update(&mut self, msg: &Self::Msg, resources: &mut Resources, state: &InternalState);
    fn view(
        &mut self,
        resources: &mut Resources,
        state: &mut InternalState,
        draw_data: &mut DrawData,
    ) -> Element<'_, Self::Msg>;
    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg>;
}

impl<T> AppCompat for T
where
    T: Sized + WhichOne,
    T: IsEitherHelper<<T as WhichOne>::Which>,
{
    type Msg = <Self as IsEitherHelper<<T as WhichOne>::Which>>::MsgHelper;
    fn init(state: &State) -> Self {
        <Self as IsEitherHelper<<T as WhichOne>::Which>>::init_helper(state)
    }
    fn update(&mut self, msg: &Self::Msg, resources: &mut Resources, state: &InternalState) {
        <Self as IsEitherHelper<<T as WhichOne>::Which>>::update_helper(self, msg, resources, state)
    }

    fn view(
        &mut self,
        resources: &mut Resources,
        state: &mut InternalState,
        draw_data: &mut DrawData,
    ) -> Element<'_, Self::Msg> {
        <Self as IsEitherHelper<<T as WhichOne>::Which>>::view_helper(
            self, resources, state, draw_data,
        )
    }

    fn subscriptions(&self, event: &Event) -> Vec<Self::Msg> {
        <Self as IsEitherHelper<<T as WhichOne>::Which>>::subscriptions_helper(self, event)
    }
}

/// Implementation detail of AppCompat
pub trait IsEitherHelper<Which>: WhichOne {
    type MsgHelper: Copy + Debug;
    fn init_helper(state: &State) -> Self;
    fn update_helper(
        &mut self,
        msg: &Self::MsgHelper,
        resources: &mut Resources,
        state: &InternalState,
    );
    fn view_helper(
        &mut self,
        resources: &mut Resources,
        state: &mut InternalState,
        draw_data: &mut DrawData,
    ) -> Element<'_, Self::MsgHelper>;
    fn subscriptions_helper(&self, event: &Event) -> Vec<Self::MsgHelper>;
}

#[derive(Clone, Copy, Debug)]
pub enum Pico8AppMsg {
    Tick,
}

impl<T> IsEitherHelper<Left> for T
where
    T: Sized + App,
{
    type MsgHelper = Pico8AppMsg;

    fn init_helper(state: &State) -> Self {
        <Self as App>::init(state)
    }

    fn update_helper(
        &mut self,
        _: &Self::MsgHelper,
        resources: &mut Resources,
        internal_state: &InternalState,
    ) {
        let state = State::new(internal_state, resources);
        <Self as App>::update(self, &state);
    }

    fn view_helper(
        &mut self,
        resources: &mut Resources,
        internal_state: &mut InternalState,
        draw_data: &mut DrawData,
    ) -> Element<'_, Self::MsgHelper> {
        let mut state = State::new(internal_state, resources);
        let mut draw = DrawContext::new(&mut state, draw_data);
        <Self as App>::draw(self, &mut draw);

        // Ugly hack
        Tree::new().into()
    }

    fn subscriptions_helper(&self, event: &Event) -> Vec<Self::MsgHelper> {
        match event {
            Event::Tick { .. } => Some(Pico8AppMsg::Tick),
            Event::Mouse(_) => None,
            Event::Keyboard(_) => None,
        }
        .into_iter()
        .collect()
    }
}

impl<T> IsEitherHelper<Right> for T
where
    T: Sized + ElmApp,
{
    type MsgHelper = <T as ElmApp>::Msg;

    fn init_helper(_: &State) -> Self {
        <T as ElmApp>::init()
    }

    fn update_helper(
        &mut self,
        msg: &Self::MsgHelper,
        resources: &mut Resources,
        _: &InternalState,
    ) {
        <T as ElmApp>::update(self, msg, resources)
    }

    fn view_helper(
        &mut self,
        resources: &mut Resources,
        _: &mut InternalState,
        _: &mut DrawData,
    ) -> Element<'_, Self::MsgHelper> {
        <T as ElmApp>::view(self, resources)
    }

    fn subscriptions_helper(&self, event: &Event) -> Vec<Self::MsgHelper> {
        <Self as ElmApp>::subscriptions(self, event)
    }
}
