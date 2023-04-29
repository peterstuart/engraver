use std::collections::HashMap;

use smufl::StaffSpaces;
use strum::EnumCount;

use super::{Alteration, Context, Step};
pub use crate::render::input::key_signature::Kind;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KeySignature {
    kind: Kind,
    num: u8,
}

impl KeySignature {
    pub fn new(kind: Kind, num: u8) -> Self {
        assert!(num >= 1 && num <= Step::COUNT as u8);

        Self { kind, num }
    }

    pub(crate) fn alterations(self) -> HashMap<Step, Alteration> {
        let alteration = match self.kind {
            Kind::Sharps => Alteration::Sharp,
            Kind::Flats => Alteration::Flat,
        };

        match self.kind {
            Kind::Sharps => [
                Step::F,
                Step::C,
                Step::G,
                Step::D,
                Step::A,
                Step::E,
                Step::B,
            ],
            Kind::Flats => [
                Step::B,
                Step::E,
                Step::A,
                Step::D,
                Step::G,
                Step::C,
                Step::F,
            ],
        }
        .into_iter()
        .map(|step| (step, alteration))
        .take(self.num as usize)
        .collect()
    }

    pub(crate) fn into_input(self, context: &mut Context) -> crate::render::input::KeySignature {
        context.start_key_signature(self);

        let octave_offset = if context.middle_c_position < StaffSpaces::zero() {
            4
        } else if context.middle_c_position >= StaffSpaces::zero()
            && context.middle_c_position <= StaffSpaces(4.0)
        {
            3
        } else {
            2
        };

        let pitches = match self.kind {
            Kind::Sharps => [
                (Step::F, 1),
                (Step::C, 1),
                (Step::G, 1),
                (Step::D, 1),
                (Step::A, 0),
                (Step::E, 1),
                (Step::B, 0),
            ],
            Kind::Flats => [
                (Step::B, 0),
                (Step::E, 1),
                (Step::A, 0),
                (Step::D, 1),
                (Step::G, 0),
                (Step::C, 1),
                (Step::F, 0),
            ],
        }
        .into_iter()
        .map(|(step, octave)| step.y(octave + octave_offset, context.middle_c_position))
        .take(self.num as usize)
        .collect();

        crate::render::input::KeySignature {
            kind: self.kind,
            pitches,
        }
    }
}
