use smufl::Glyph;

use crate::render::StemDirection;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Duration {
    pub value: Value,
    pub dots: Option<Dots>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Value {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
}

impl Value {
    pub fn notehead_glyph(&self) -> Glyph {
        match self {
            Self::Whole => Glyph::NoteheadWhole,
            Self::Half => Glyph::NoteheadHalf,
            _ => Glyph::NoteheadBlack,
        }
    }

    pub fn flag_glyph(&self, stem_direction: StemDirection) -> Option<Glyph> {
        match (self, stem_direction) {
            (Self::Whole, _) => None,
            (Self::Half, _) => None,
            (Self::Quarter, _) => None,
            (Self::Eighth, StemDirection::Up) => Some(Glyph::Flag8thUp),
            (Self::Eighth, StemDirection::Down) => Some(Glyph::Flag8thDown),
            (Self::Sixteenth, StemDirection::Up) => Some(Glyph::Flag16thUp),
            (Self::Sixteenth, StemDirection::Down) => Some(Glyph::Flag16thDown),
            (Self::ThirtySecond, StemDirection::Up) => Some(Glyph::Flag32ndUp),
            (Self::ThirtySecond, StemDirection::Down) => Some(Glyph::Flag32ndDown),
            (Self::SixtyFourth, StemDirection::Up) => Some(Glyph::Flag64thUp),
            (Self::SixtyFourth, StemDirection::Down) => Some(Glyph::Flag64thDown),
        }
    }

    pub fn rest_glyph(&self) -> Glyph {
        match self {
            Value::Whole => Glyph::RestWhole,
            Value::Half => Glyph::RestHalf,
            Value::Quarter => Glyph::RestQuarter,
            Value::Eighth => Glyph::Rest8th,
            Value::Sixteenth => Glyph::Rest16th,
            Value::ThirtySecond => Glyph::Rest32nd,
            Value::SixtyFourth => Glyph::Rest64th,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Dots {
    Dot,
    DoubleDot,
}
