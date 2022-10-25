use smufl::{Metadata, StaffSpaces};

use super::Duration;
use crate::render::{
    ir::{Coord, Element, Symbol},
    metadata_extensions::MetadataExtensions,
    Output, Render,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Rest {
    pub duration: Duration,
}

impl Render for Rest {
    fn render(&self, x: StaffSpaces, metadata: &Metadata) -> crate::render::Output {
        let glyph = self.duration.value.rest_glyph();

        let element = Element::Symbol(Symbol {
            origin: Coord {
                x,
                y: StaffSpaces(2.0),
            },
            value: glyph.codepoint(),
        });
        let width = metadata.width_of(glyph);

        Output {
            elements: vec![element],
            width,
        }
    }
}
