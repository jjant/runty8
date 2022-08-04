use super::Msg;
use crate::ui::text::Text;
use crate::ui::{DrawFn, Element, Tree};

#[derive(Clone, Copy)]
enum BasicNote {
    C = 0,
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
    let note_bars: Vec<_> = sound
        .notes
        .iter()
        .enumerate()
        .map(|(index, (note, _volume))| note_bar(index, note))
        .collect();

    Tree::with_children(note_bars)
        .push(Text::new("THIS IS THE SOUND EDITOR", 20, 30, 7))
        .into()
}

fn note_bar<'a>(index: usize, note: &Note) -> Element<'a, Msg> {
    let y_offset = note.octave as i32 * 12 + note.note as i32;

    let padding = 1;
    let width = 2;
    let x = index as i32 * 4 + padding;
    let y = 80 - y_offset;

    DrawFn::new(move |draw| {
        draw.rectfill(x, y, x + width - 1, 80, 1);
        draw.rectfill(x, y, x + width - 1, y + width - 1, 8);
    })
    .into()
}
