use smufl::{Glyph, Metadata, StaffSpaces};

use super::{duration, Accidental, Beam, Duration};
use crate::{
    render::{
        context::{beam, Context},
        engraving_defaults_extensions::EngravingDefaultsExtensions,
        glyph_data_extensions::GlyphDataExtensions,
        ir::{Coord, Element, Group, Line, Linecap, Symbol},
        metadata_extensions::MetadataExtensions,
        stem::{self, Stem},
        Output, Render,
    },
    Result,
};

pub const DEFAULT_ACCIDENTAL_SPACING: StaffSpaces = StaffSpaces(0.3);

#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub y: StaffSpaces,
    pub accidental: Option<Accidental>,
    pub duration: Duration,
    pub beam: Option<Beam>,
    pub id: Option<String>,
}

impl Render for Note {
    fn render(&self, x: StaffSpaces, context: &mut Context, metadata: &Metadata) -> Result<Output> {
        let glyph = self.duration.value.notehead_glyph();

        let notehead = create_notehead(x, self.y, glyph);

        let mut elements = vec![notehead];

        let mut leger_lines = create_leger_lines(x, self.y, glyph, metadata)?;
        elements.append(&mut leger_lines);

        if let Some(accidental) = self.accidental {
            let glyph = accidental.glyph();
            let accidental = create_accidental(x, self.y, glyph, metadata)?;
            elements.push(accidental);
        }

        match context.beam() {
            Some(beam) => beam.add_notehead(beam::Notehead {
                glyph,
                x,
                y: self.y,
                min_stem_length: None,
            }),
            None => {
                if self.duration.value != duration::Value::Whole {
                    let stem_direction = if self.y >= StaffSpaces(2.0) {
                        stem::Direction::Down
                    } else {
                        stem::Direction::Up
                    };

                    let stem = Stem::new(glyph, x, self.y, stem_direction, None);

                    if let Some(flag_glyph) = self.duration.value.flag_glyph(stem_direction) {
                        let flag = create_flag(
                            x,
                            stem.end(),
                            glyph,
                            flag_glyph,
                            stem_direction,
                            metadata,
                        )?;
                        elements.push(flag);
                    }

                    elements.push(stem.render(metadata)?);
                }
            }
        };

        let width = metadata.width_of(glyph)?;

        if self.id.is_some() {
            Ok(Output {
                elements: vec![Element::Group(Group {
                    id: self.id.clone(),
                    elements,
                })],
                width,
            })
        } else {
            Ok(Output { elements, width })
        }
    }
}

pub fn create_notehead(x: StaffSpaces, y: StaffSpaces, glyph: Glyph) -> Element<StaffSpaces> {
    Element::Symbol(Symbol {
        origin: Coord { x, y },
        value: glyph.codepoint(),
    })
}

pub fn create_leger_lines(
    x: StaffSpaces,
    y: StaffSpaces,
    glyph: Glyph,
    metadata: &Metadata,
) -> Result<Vec<Element<StaffSpaces>>> {
    if y >= StaffSpaces(0.0) && y <= StaffSpaces(4.0) {
        return Ok(vec![]);
    }

    let (num_lines, y, increment) = if y >= StaffSpaces(4.0) {
        (
            (y - StaffSpaces(4.0)).0 as usize,
            StaffSpaces(5.0),
            StaffSpaces(1.0),
        )
    } else {
        ((y * -1.0).0 as usize, StaffSpaces(-1.0), StaffSpaces(-1.0))
    };

    let left_x = x + metadata.engraving_defaults.leger_line_extension() * -1.0;
    let right_x =
        x + metadata.width_of(glyph)? + metadata.engraving_defaults.leger_line_extension();

    Ok((0..num_lines)
        .map(|num| {
            let y = y + increment * (num as f64);
            Element::Line(Line {
                from: Coord { x: left_x, y },
                to: Coord { x: right_x, y },
                thickness: metadata.engraving_defaults.leger_line_thickness(),
                cap: Linecap::Round,
            })
        })
        .collect())
}

pub fn create_accidental(
    x: StaffSpaces,
    y: StaffSpaces,
    accidental_glyph: Glyph,
    metadata: &Metadata,
) -> Result<Element<StaffSpaces>> {
    let x = x - DEFAULT_ACCIDENTAL_SPACING - metadata.width_of(accidental_glyph)?;

    Ok(Element::Symbol(Symbol {
        origin: Coord { x, y },
        value: accidental_glyph.codepoint(),
    }))
}

pub fn create_flag(
    x: StaffSpaces,
    stem_end: StaffSpaces,
    notehead_glyph: Glyph,
    flag_glyph: Glyph,
    stem_direction: stem::Direction,
    metadata: &Metadata,
) -> Result<Element<StaffSpaces>> {
    let anchors = metadata.anchors.try_get(notehead_glyph)?;

    let (x, y) = match stem_direction {
        stem::Direction::Up => {
            let se_anchor = anchors.stem_up_se.unwrap_or_else(|| {
                panic!("{notehead_glyph:?} should have stem_up_se anchor defined")
            });
            let stem_thickness = metadata.engraving_defaults.stem_thickness();

            (x + se_anchor.x() - stem_thickness, stem_end)
        }
        stem::Direction::Down => (x, stem_end),
    };

    Ok(Element::Symbol(Symbol {
        origin: Coord { x, y },
        value: flag_glyph.codepoint(),
    }))
}
