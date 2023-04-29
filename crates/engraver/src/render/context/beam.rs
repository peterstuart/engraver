use smufl::{Glyph, Metadata, StaffSpaces};

use super::Context;
use crate::{
    render::{
        engraving_defaults_extensions::EngravingDefaultsExtensions,
        ir::{Coord, Element, Polygon},
        stem::{self, Stem},
        Output, Render,
    },
    Result,
};

#[derive(Debug)]
pub struct Notehead {
    pub glyph: Glyph,
    pub x: StaffSpaces,
    pub y: StaffSpaces,
    pub min_stem_length: Option<StaffSpaces>,
}

#[derive(Debug)]
pub struct Beam {
    pub stem_direction: stem::Direction,
    noteheads: Vec<Notehead>,
}

impl Beam {
    pub fn new(stem_direction: stem::Direction) -> Self {
        Self {
            stem_direction,
            noteheads: vec![],
        }
    }

    pub fn add_notehead(&mut self, notehead: Notehead) {
        self.noteheads.push(notehead);
    }
}

impl Render for Beam {
    fn render(
        &self,
        _x: StaffSpaces,
        _context: &mut Context,
        metadata: &Metadata,
    ) -> Result<Output> {
        let beam_thickness = metadata.engraving_defaults.beam_thickness();

        let mut stems: Vec<_> = self
            .noteheads
            .iter()
            .map(|notehead| {
                Stem::new(
                    notehead.glyph,
                    notehead.x,
                    notehead.y,
                    self.stem_direction,
                    notehead.min_stem_length,
                )
            })
            .collect();

        let first_stem = stems.first().unwrap();
        let last_stem = stems.last().unwrap();

        let start_x = first_stem.left(metadata)?;
        let end_x = last_stem.right(metadata)?;

        let start_y = first_stem.end();
        let end_y = last_stem.end();

        for stem in &mut stems {
            stem.adjust_length(beam_thickness / -2.0);
        }

        let mut elements = stems
            .iter()
            .map(|stem| stem.render(metadata))
            .collect::<Result<Vec<_>>>()?;

        let polygon = Polygon::new(&[
            Coord {
                x: start_x,
                y: start_y,
            },
            Coord { x: end_x, y: end_y },
            Coord {
                x: end_x,
                y: end_y - beam_thickness,
            },
            Coord {
                x: start_x,
                y: start_y - beam_thickness,
            },
        ]);
        let element = Element::Polygon(polygon);
        elements.push(element);

        Ok(Output {
            elements,
            width: StaffSpaces::zero(),
        })
    }
}
