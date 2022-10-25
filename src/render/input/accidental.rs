use smufl::Glyph;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
