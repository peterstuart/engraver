use smufl::Glyph;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, Eq, Ord, PartialEq, PartialOrd)]
pub enum Accidental {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Accidental {
    pub fn glyph(&self) -> Glyph {
        match self {
            Accidental::DoubleFlat => Glyph::AccidentalDoubleFlat,
            Accidental::Flat => Glyph::AccidentalFlat,
            Accidental::Natural => Glyph::AccidentalNatural,
            Accidental::Sharp => Glyph::AccidentalSharp,
            Accidental::DoubleSharp => Glyph::AccidentalDoubleSharp,
        }
    }
}
