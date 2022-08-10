use super::Msg;
use crate::ui::button::{self, Button};
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

const NOTES: &'static [BasicNote] = {
    use BasicNote::*;
    &[C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B]
};

#[derive(Clone, Copy)]
struct Note {
    note: BasicNote,
    octave: u8, // Actually goes from 0 to 5
}

impl Note {
    fn y_offset(&self) -> i32 {
        2 * (self.octave as i32 * 12 + self.note as i32)
    }

    fn available_notes() -> Vec<Self> {
        (0..5)
            .flat_map(|octave| NOTES.iter().copied().map(move |note| Self { note, octave }))
            .collect()
    }
}

// Range is actually 0..=7
// TODO: Pack later maybe?
type Volume = u8;

pub(crate) struct Sound {
    notes: [(Note, Volume); SoundEditor::NOTES],
}

impl Sound {
    pub fn new() -> Self {
        let notes = [(
            Note {
                note: BasicNote::D,
                octave: 3,
            },
            5,
        ); SoundEditor::NOTES];

        Self { notes }
    }
}

#[derive(Debug)]
pub(crate) struct SoundEditor {
    buttons: Vec<Vec<button::State>>,
}

impl SoundEditor {
    const NOTES: usize = 32;

    pub(crate) fn new() -> Self {
        Self {
            buttons: vec![vec![button::State::new(); Note::available_notes().len()]; Self::NOTES],
        }
    }
}

pub(crate) fn view<'a>(sound_editor: &'a mut SoundEditor, sound: &Sound) -> Element<'a, Msg> {
    let note_bars: Vec<_> = sound
        .notes
        .iter()
        .zip(sound_editor.buttons.iter_mut())
        .enumerate()
        .map(|(index, ((note, _volume), button))| note_bar(button, index, note))
        .collect();

    Tree::with_children(note_bars)
        .push(Text::new("THIS IS THE SOUND EDITOR", 20, 30, 7))
        .into()
}

fn note_bar<'a, Msg: std::fmt::Debug + Copy + 'a>(
    button_states: &'a mut [button::State],
    index: usize,
    note: &Note,
    //on_click: &mut impl FnMut() -> Msg,
) -> Element<'a, Msg> {
    let y_offset = note.y_offset();

    let padding = 1;
    let width = 2;
    let x = index as i32 * 4 + padding;
    let y = 80 - y_offset;

    let mut buttons = vec![];
    for (note, button_state) in Note::available_notes().into_iter().zip(button_states) {
        buttons.push(
            Button::new(
                x,
                y + note.y_offset(),
                2,
                2,
                None, //              Some(on_click()),
                button_state,
                DrawFn::new(|draw| {
                    draw.pset(0, 0, 7);
                }),
            )
            .into(),
        );
    }

    Tree::with_children(buttons).into()
}
