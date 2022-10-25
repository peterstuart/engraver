use smufl::{Metadata, StaffSpaces};

use super::{Clef, KeySignature, Measure, TimeSignature};
use crate::render::{
    engraving_defaults_extensions::EngravingDefaultsExtensions,
    ir::{Coord, Element, Line, Linecap},
    Renderer,
};

const BEGINNING_OF_STAFF_SPACE: StaffSpaces = StaffSpaces(1.0);
const SPACE_AFTER_CLEF: StaffSpaces = StaffSpaces(1.0);
const SPACE_AFTER_KEY_SIGNATURE: StaffSpaces = StaffSpaces(1.0);
const SPACE_AFTER_TIME_SIGNATURE: StaffSpaces = StaffSpaces(1.0);

const NUM_STAFF_LINES: u32 = 5;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Staff {
    pub clef: Option<Clef>,
    pub key_signature: Option<KeySignature>,
    pub time_signature: Option<TimeSignature>,
    pub measures: Vec<Measure>,
}

impl Staff {
    pub fn render(&self, metadata: &Metadata) -> Vec<Element<StaffSpaces>> {
        let mut renderer = Renderer::new(metadata);

        renderer.advance(BEGINNING_OF_STAFF_SPACE);

        if let Some(clef) = &self.clef {
            renderer.render(clef);
            renderer.advance(SPACE_AFTER_CLEF);
        }

        if let Some(key_signature) = &self.key_signature {
            renderer.render(key_signature);
            renderer.advance(SPACE_AFTER_KEY_SIGNATURE);
        }

        if let Some(time_signature) = &self.time_signature {
            renderer.render(time_signature);
            renderer.advance(SPACE_AFTER_TIME_SIGNATURE);
        }

        for measure in &self.measures {
            measure.render(&mut renderer);
        }

        renderer.add_elements(Self::staff_lines(renderer.position(), metadata));

        renderer.to_elements()
    }

    fn staff_lines(length: StaffSpaces, metadata: &Metadata) -> Vec<Element<StaffSpaces>> {
        let staff_line_thickness = metadata.engraving_defaults.staff_line_thickness();

        (0..NUM_STAFF_LINES)
            .map(move |number| {
                Element::Line(Line {
                    from: Coord {
                        x: StaffSpaces::zero(),
                        y: number.into(),
                    },
                    to: Coord {
                        x: length,
                        y: number.into(),
                    },
                    thickness: staff_line_thickness,
                    cap: Linecap::Butt,
                })
            })
            .collect()
    }
}
