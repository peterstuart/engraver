use smufl::Glyph;
use strum_macros::EnumIter;

use crate::render::stem;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Duration {
    pub value: Value,
    pub dots: Option<Dots>,
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, Ord, PartialEq, PartialOrd)]
pub enum Value {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
    OneHundredTwentyEighth,
    TwoHundredFiftySixth,
}

impl Value {
    pub fn notehead_glyph(&self) -> Glyph {
        match self {
            Self::Whole => Glyph::NoteheadWhole,
            Self::Half => Glyph::NoteheadHalf,
            _ => Glyph::NoteheadBlack,
        }
    }

    pub fn flag_glyph(&self, stem_direction: stem::Direction) -> Option<Glyph> {
        match (self, stem_direction) {
            (Self::Whole, _) => None,
            (Self::Half, _) => None,
            (Self::Quarter, _) => None,
            (Self::Eighth, stem::Direction::Up) => Some(Glyph::Flag8thUp),
            (Self::Eighth, stem::Direction::Down) => Some(Glyph::Flag8thDown),
            (Self::Sixteenth, stem::Direction::Up) => Some(Glyph::Flag16thUp),
            (Self::Sixteenth, stem::Direction::Down) => Some(Glyph::Flag16thDown),
            (Self::ThirtySecond, stem::Direction::Up) => Some(Glyph::Flag32ndUp),
            (Self::ThirtySecond, stem::Direction::Down) => Some(Glyph::Flag32ndDown),
            (Self::SixtyFourth, stem::Direction::Up) => Some(Glyph::Flag64thUp),
            (Self::SixtyFourth, stem::Direction::Down) => Some(Glyph::Flag64thDown),
            (Self::OneHundredTwentyEighth, stem::Direction::Up) => Some(Glyph::Flag128thUp),
            (Self::OneHundredTwentyEighth, stem::Direction::Down) => Some(Glyph::Flag128thDown),
            (Self::TwoHundredFiftySixth, stem::Direction::Up) => Some(Glyph::Flag256thUp),
            (Self::TwoHundredFiftySixth, stem::Direction::Down) => Some(Glyph::Flag256thDown),
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
            Value::OneHundredTwentyEighth => Glyph::Rest128th,
            Value::TwoHundredFiftySixth => Glyph::Rest256th,
        }
    }
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, Ord, PartialEq, PartialOrd)]
pub enum Dots {
    Dot,
    DoubleDot,
}
