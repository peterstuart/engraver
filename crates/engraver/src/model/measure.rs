use super::{Barline, Chord, Context, Note, Rest};
use crate::render;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Measure {
    pub elements: Vec<Element>,
    pub barline: Barline,
}

impl Measure {
    pub(crate) fn into_input(self, context: &mut Context) -> render::input::Measure {
        context.start_measure();

        render::input::Measure {
            elements: self
                .elements
                .into_iter()
                .map(|element| element.into_input(context))
                .collect(),
            barline: self.barline,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    Note(Note),
    Chord(Chord),
    Rest(Rest),
}

impl Element {
    pub(crate) fn into_input(self, context: &mut Context) -> render::input::measure::Element {
        match self {
            Element::Note(note) => render::input::measure::Element::Note(note.into_input(context)),
            Element::Chord(chord) => {
                render::input::measure::Element::Chord(chord.into_input(context))
            }
            Element::Rest(rest) => render::input::measure::Element::Rest(rest),
        }
    }
}
