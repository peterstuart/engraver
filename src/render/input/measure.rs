use smufl::StaffSpaces;

use super::{duration, Barline, Chord, Duration, Note, Rest};
use crate::render::{Render, Renderer};

const BEGINNING_OF_MEASURE_SPACE: StaffSpaces = StaffSpaces(2.0);
const END_OF_MEASURE_SPACE: StaffSpaces = StaffSpaces(2.0);
const BASE_SPACE: StaffSpaces = StaffSpaces(8.0);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Measure {
    pub elements: Vec<Element>,
    pub bar_line: Barline,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Element {
    Note(Note),
    Chord(Chord),
    Rest(Rest),
}

impl Element {
    fn duration(&self) -> Duration {
        match self {
            Element::Note(note) => note.duration,
            Element::Chord(chord) => chord.duration,
            Element::Rest(rest) => rest.duration,
        }
    }

    fn spacing(&self) -> StaffSpaces {
        let duration = self.duration();

        let value_multiplier = match duration.value {
            super::duration::Value::Whole => 1.0,
            super::duration::Value::Half => 1.0 / 2.0,
            super::duration::Value::Quarter => 1.0 / 4.0,
            super::duration::Value::Eighth => 1.0 / 8.0,
            super::duration::Value::Sixteenth => 1.0 / 16.0,
            super::duration::Value::ThirtySecond => 1.0 / 32.0,
            super::duration::Value::SixtyFourth => 1.0 / 64.0,
        };

        let dots_multiplier = match duration.dots {
            Some(duration::Dots::Dot) => 1.5,
            Some(duration::Dots::DoubleDot) => 1.75,
            None => 1.0,
        };

        BASE_SPACE * value_multiplier * dots_multiplier
    }
}

impl Render for Element {
    fn render(&self, x: smufl::StaffSpaces, metadata: &smufl::Metadata) -> crate::render::Output {
        match self {
            Element::Note(note) => note.render(x, metadata),
            Element::Chord(chord) => chord.render(x, metadata),
            Element::Rest(rest) => rest.render(x, metadata),
        }
    }
}

impl Measure {
    pub fn render(&self, renderer: &mut Renderer) {
        renderer.advance(BEGINNING_OF_MEASURE_SPACE);

        for element in &self.elements {
            renderer.render(element);
            renderer.advance(element.spacing());
        }

        renderer.advance(END_OF_MEASURE_SPACE);

        renderer.render(&self.bar_line);
    }
}
