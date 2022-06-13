use crate::Msg;

use super::{enemy::Enemy, item::Item};
use enum_dispatch::enum_dispatch;
use runty8::ui::Element;

#[enum_dispatch(EntityT)]
pub enum Entity {
    Enemy,
    Item,
}

#[enum_dispatch]
pub trait EntityT {
    fn update(&mut self) -> ShouldDestroy;
    fn view(&self) -> Element<'_, Msg>;
}

pub struct UpdateAction {
    entities: Vec<Entity>,
    should_destroy: ShouldDestroy,
}

#[derive(PartialEq, Debug)]
pub enum ShouldDestroy {
    No,
    Yes,
}
