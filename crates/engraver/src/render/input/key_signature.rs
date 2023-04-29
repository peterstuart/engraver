use smufl::{Glyph, Metadata, StaffSpaces};

use crate::{
    render::{
        context::Context,
        ir::{Coord, Element, Symbol},
        metadata_extensions::MetadataExtensions,
        Output, Render,
    },
    Result,
};

const SPACE_AFTER_SYMBOL: StaffSpaces = StaffSpaces(0.1);

#[derive(Clone, Debug, PartialEq)]
pub struct KeySignature {
    pub kind: Kind,
    pub pitches: Vec<StaffSpaces>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    Sharps,
    Flats,
}

impl Default for Kind {
    fn default() -> Self {
        Self::Sharps
    }
}

impl Kind {
    fn glyph(&self) -> Glyph {
        match self {
            Self::Sharps => Glyph::AccidentalSharp,
            Self::Flats => Glyph::AccidentalFlat,
        }
    }
}

impl Render for KeySignature {
    fn render(
        &self,
        x: StaffSpaces,
        _context: &mut Context,
        metadata: &Metadata,
    ) -> Result<Output> {
        let glyph = self.kind.glyph();
        let codepoint = glyph.codepoint();
        let symbol_width = metadata.width_of(glyph)? + SPACE_AFTER_SYMBOL;

        let elements = self
            .pitches
            .iter()
            .enumerate()
            .map(|(index, pitch)| {
                Element::Symbol(Symbol {
                    origin: Coord {
                        x: x + symbol_width * (index as f64),
                        y: *pitch,
                    },
                    value: codepoint,
                })
            })
            .collect();

        let width = symbol_width * (self.pitches.len() as f64);

        Ok(Output { elements, width })
    }
}
