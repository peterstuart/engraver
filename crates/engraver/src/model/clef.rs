use smufl::{Glyph, StaffSpaces};

use super::Context;
use crate::render;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Clef {
    Treble,
    Alto,
    Tenor,
    Bass,
}

impl Default for Clef {
    fn default() -> Self {
        Self::Treble
    }
}

impl Clef {
    pub(crate) fn into_input(self, context: &mut Context) -> render::input::Clef {
        let (glyph, y, middle_c_position) = match self {
            Clef::Treble => (Glyph::GClef, StaffSpaces(1.0), StaffSpaces(-1.0)),
            Clef::Alto => (Glyph::CClef, StaffSpaces(2.0), StaffSpaces(2.0)),
            Clef::Tenor => (Glyph::CClef, StaffSpaces(3.0), StaffSpaces(3.0)),
            Clef::Bass => (Glyph::FClef, StaffSpaces(3.0), StaffSpaces(5.0)),
        };

        context.middle_c_position = middle_c_position;

        render::input::Clef { glyph, y }
    }
}
