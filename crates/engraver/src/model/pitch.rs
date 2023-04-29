use smufl::StaffSpaces;
use strum_macros::EnumCount;

use super::{context::AccidentalState, Context};
use crate::render::input::Accidental;

#[derive(Clone, Copy, Debug, EnumCount, Eq, Hash, PartialEq)]
pub enum Step {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Step {
    pub(crate) fn y(&self, octave: i8, middle_c_position: StaffSpaces) -> StaffSpaces {
        StaffSpaces(match self {
            Self::C => 0.0,
            Self::D => 0.5,
            Self::E => 1.0,
            Self::F => 1.5,
            Self::G => 2.0,
            Self::A => 2.5,
            Self::B => 3.0,
        }) + OCTAVE_IN_STAFF_SPACES * ((octave - 4) as f64)
            + middle_c_position
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Alteration {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Default for Alteration {
    fn default() -> Self {
        Self::Natural
    }
}

impl From<Alteration> for Accidental {
    fn from(value: Alteration) -> Self {
        match value {
            Alteration::DoubleFlat => Accidental::DoubleFlat,
            Alteration::Flat => Accidental::Flat,
            Alteration::Natural => Accidental::Natural,
            Alteration::Sharp => Accidental::Sharp,
            Alteration::DoubleSharp => Accidental::DoubleSharp,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pitch {
    pub step: Step,
    pub alteration: Alteration,
    pub octave: i8,
}

const OCTAVE_IN_STAFF_SPACES: StaffSpaces = StaffSpaces(3.5);

impl Pitch {
    pub(crate) fn into_input(self, context: &mut Context) -> (StaffSpaces, Option<Accidental>) {
        let y = self.step.y(self.octave, context.middle_c_position);

        let accidental = match context.add_pitch(self) {
            AccidentalState::NeedsAccidental => Some(self.alteration.into()),
            AccidentalState::NoAccidental => None,
        };

        (y, accidental)
    }
}
