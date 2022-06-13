use crate::Msg;

use super::{enemy::Enemy, item::DroppedItem};
use enum_dispatch::enum_dispatch;
use runty8::ui::Element;

#[enum_dispatch(EntityT)]
pub enum Entity {
    Enemy,
    DroppedItem,
}

#[enum_dispatch]
pub trait EntityT {
    fn update(&mut self) -> UpdateAction;
    fn view(&self) -> Element<'_, Msg>;
}

pub struct UpdateAction {
    pub entities: Vec<Entity>,
    pub should_destroy: ShouldDestroy,
}

#[derive(PartialEq, Debug)]
pub enum ShouldDestroy {
    No,
    Yes,
}
