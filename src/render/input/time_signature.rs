use smufl::{Glyph, Metadata, StaffSpaces};

use crate::render::{
    ir::{Coord, Element, Symbol},
    metadata_extensions::MetadataExtensions,
    Output, Render,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeSignature {
    pub numerator: u8,
    pub denominator: u8,
}

impl TimeSignature {
    fn numerator_glyphs(&self) -> Vec<Glyph> {
        Self::glyphs_for_number(self.numerator)
    }

    fn denominator_glyphs(&self) -> Vec<Glyph> {
        Self::glyphs_for_number(self.denominator)
    }

    fn glyphs_for_number(number: u8) -> Vec<Glyph> {
        number
            .to_string()
            .chars()
            .map(Self::glyph_for_char)
            .collect()
    }

    fn glyph_for_char(c: char) -> Glyph {
        match c {
            '0' => Glyph::TimeSig0,
            '1' => Glyph::TimeSig1,
            '2' => Glyph::TimeSig2,
            '3' => Glyph::TimeSig3,
            '4' => Glyph::TimeSig4,
            '5' => Glyph::TimeSig5,
            '6' => Glyph::TimeSig6,
            '7' => Glyph::TimeSig7,
            '8' => Glyph::TimeSig8,
            '9' => Glyph::TimeSig9,
            _ => unreachable!("There should be no non-digit chars"),
        }
    }

    fn render_glyphs(
        glyphs: &[Glyph],
        x: StaffSpaces,
        y: StaffSpaces,
        mut offset: StaffSpaces,
        metadata: &Metadata,
    ) -> Vec<Element<StaffSpaces>> {
        let mut elements = Vec::with_capacity(glyphs.len());

        for glyph in glyphs {
            let x = x + offset;

            elements.push(Element::Symbol(Symbol {
                origin: Coord { x, y },
                value: glyph.codepoint(),
            }));

            offset += metadata.width_of(*glyph);
        }

        elements
    }
}

impl Render for TimeSignature {
    fn render(&self, x: StaffSpaces, metadata: &Metadata) -> Output {
        let numerator_width: StaffSpaces = self
            .numerator_glyphs()
            .into_iter()
            .map(|glyph| metadata.width_of(glyph))
            .sum();
        let denominator_width = self
            .denominator_glyphs()
            .into_iter()
            .map(|glyph| metadata.width_of(glyph))
            .sum();

        let width = numerator_width.max(denominator_width);

        let numerator_glyphs = self.numerator_glyphs();
        let denominator_glyphs = self.denominator_glyphs();

        let mut elements = Vec::with_capacity(numerator_glyphs.len() + denominator_glyphs.len());

        let numerator_offset = (width - numerator_width) / 2.0;
        elements.append(&mut Self::render_glyphs(
            &numerator_glyphs,
            x,
            StaffSpaces(3.0),
            numerator_offset,
            metadata,
        ));

        let denominator_offset = (width - denominator_width) / 2.0;
        elements.append(&mut Self::render_glyphs(
            &denominator_glyphs,
            x,
            StaffSpaces(1.0),
            denominator_offset,
            metadata,
        ));

        Output { elements, width }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numerator_glyphs() {
        assert_eq!(
            TimeSignature {
                numerator: 12,
                denominator: 8
            }
            .numerator_glyphs(),
            [Glyph::TimeSig1, Glyph::TimeSig2]
        );
    }

    #[test]
    fn denominator_glyphs() {
        assert_eq!(
            TimeSignature {
                numerator: 12,
                denominator: 8
            }
            .denominator_glyphs(),
            [Glyph::TimeSig8]
        );
    }
}
