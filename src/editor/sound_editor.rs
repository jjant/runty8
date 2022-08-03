use super::Msg;
use crate::ui::text::Text;
use crate::ui::Element;

pub(crate) fn view<'a>() -> Element<'a, Msg> {
    Text::new("THIS IS THE SOUND EDITOR", 20, 30, 7).into()
}
