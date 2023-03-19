use smufl::{Glyph, StaffSpaces};

use crate::{
    render::{
        ir::{Coord, Element, Symbol},
        metadata_extensions::MetadataExtensions,
        Output, Render,
    },
    Result,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Clef {
    pub glyph: Glyph,
    pub y: StaffSpaces,
}

impl Render for Clef {
    fn render(&self, x: StaffSpaces, metadata: &smufl::Metadata) -> Result<Output> {
        Ok(Output {
            elements: vec![Element::Symbol(Symbol {
                origin: Coord { x, y: self.y },
                value: self.glyph.codepoint(),
            })],
            width: metadata.width_of(self.glyph)?,
        })
    }
}
