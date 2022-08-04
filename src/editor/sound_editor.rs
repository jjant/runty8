use super::Msg;
use crate::ui::text::Text;
use crate::ui::Element;

#[derive(Clone, Copy)]
enum BasicNote {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

#[derive(Clone, Copy)]
struct Note {
    note: BasicNote,
    octave: u8, // Actually goes from 0 to 5
}

// Range is actually 0..=7
// TODO: Pack later maybe?
type Volume = u8;

pub(crate) struct Sound {
    notes: [(Note, Volume); 32],
}

impl Sound {
    pub fn new() -> Self {
        let mut notes = [(
            Note {
                note: BasicNote::D,
                octave: 3,
            },
            5,
        ); 32];

        notes[15] = (
            Note {
                note: BasicNote::A,
                octave: 4,
            },
            5,
        );
        Self { notes }
    }
}

pub(crate) fn view<'a>(sound: &Sound) -> Element<'a, Msg> {
    Text::new("THIS IS THE SOUND EDITOR", 20, 30, 7).into()
}
