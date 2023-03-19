use smufl::{Glyph, Metadata, StaffSpaces};

use super::{duration, Accidental, Duration};
use crate::render::{
    engraving_defaults_extensions::EngravingDefaultsExtensions,
    ir::{Coord, Element, Line, Linecap, Polygon, Size, Symbol},
    metadata_extensions::MetadataExtensions,
    stem_direction::StemDirection,
    Output, Render,
};

pub const DEFAULT_ACCIDENTAL_SPACING: StaffSpaces = StaffSpaces(0.3);
pub const DEFAULT_STEM_LENGTH: StaffSpaces = StaffSpaces(3.5);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Note {
    pub y: StaffSpaces,
    pub accidental: Option<Accidental>,
    pub duration: Duration,
}

impl Render for Note {
    fn render(&self, x: StaffSpaces, metadata: &Metadata) -> crate::render::Output {
        let glyph = self.duration.value.notehead_glyph();

        let notehead = create_notehead(x, self.y, glyph);

        let mut elements = vec![notehead];

        let mut leger_lines = create_leger_lines(x, self.y, glyph, metadata);
        elements.append(&mut leger_lines);

        if let Some(accidental) = self.accidental {
            let glyph = accidental.glyph();
            let accidental = create_accidental(x, self.y, glyph, metadata);
            elements.push(accidental);
        }

        if self.duration.value != duration::Value::Whole {
            let stem_direction = if self.y >= StaffSpaces(2.0) {
                StemDirection::Down
            } else {
                StemDirection::Up
            };

            if let Some(flag_glyph) = self.duration.value.flag_glyph(stem_direction) {
                let stem_end = match stem_direction {
                    StemDirection::Up => self.y + DEFAULT_STEM_LENGTH,
                    StemDirection::Down => self.y - DEFAULT_STEM_LENGTH,
                };

                let flag = create_flag(x, stem_end, glyph, flag_glyph, stem_direction, metadata);
                elements.push(flag);
            }

            let (_, stem) = create_stem(
                x,
                self.y,
                DEFAULT_STEM_LENGTH,
                stem_direction,
                glyph,
                metadata,
            );
            elements.push(stem);
        }

        let width = metadata.width_of(glyph);

        Output { elements, width }
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
) -> Vec<Element<StaffSpaces>> {
    if y >= StaffSpaces(0.0) && y <= StaffSpaces(4.0) {
        return vec![];
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
    let right_x = x + metadata.width_of(glyph) + metadata.engraving_defaults.leger_line_extension();

    (0..num_lines)
        .map(|num| {
            let y = y + increment * (num as f64);
            Element::Line(Line {
                from: Coord { x: left_x, y },
                to: Coord { x: right_x, y },
                thickness: metadata.engraving_defaults.leger_line_thickness(),
                cap: Linecap::Round,
            })
        })
        .collect()
}

pub fn create_accidental(
    x: StaffSpaces,
    y: StaffSpaces,
    accidental_glyph: Glyph,
    metadata: &Metadata,
) -> Element<StaffSpaces> {
    let x = x - DEFAULT_ACCIDENTAL_SPACING - metadata.width_of(accidental_glyph);

    Element::Symbol(Symbol {
        origin: Coord { x, y },
        value: accidental_glyph.codepoint(),
    })
}

pub fn create_flag(
    x: StaffSpaces,
    stem_end: StaffSpaces,
    notehead_glyph: Glyph,
    flag_glyph: Glyph,
    stem_direction: StemDirection,
    metadata: &Metadata,
) -> Element<StaffSpaces> {
    let anchors = metadata.anchors.get(notehead_glyph);

    let (x, y) = match stem_direction {
        StemDirection::Up => {
            let se_anchor = anchors.stem_up_se.unwrap_or_else(|| {
                panic!("{notehead_glyph:?} should have stem_up_se anchor defined")
            });
            let stem_thickness = metadata.engraving_defaults.stem_thickness();

            (x + se_anchor.x() - stem_thickness, stem_end)
        }
        StemDirection::Down => (x, stem_end),
    };

    Element::Symbol(Symbol {
        origin: Coord { x, y },
        value: flag_glyph.codepoint(),
    })
}

pub fn create_stem(
    x: StaffSpaces,
    y: StaffSpaces,
    length: StaffSpaces,
    stem_direction: StemDirection,
    glyph: Glyph,
    metadata: &Metadata,
) -> (StaffSpaces, Element<StaffSpaces>) {
    let anchors = metadata.anchors.get(glyph);

    let stem_thickness = metadata.engraving_defaults.stem_thickness();
    let (stem_end, stem_polygon) = match stem_direction {
        StemDirection::Up => {
            let se_anchor = anchors
                .stem_up_se
                .unwrap_or_else(|| panic!("{glyph:?} should have stem_up_se anchor defined"));

            let origin = Coord {
                x: x + se_anchor.x() - stem_thickness,
                y: y + se_anchor.y(),
            };

            let size = Size {
                width: stem_thickness,
                height: length - se_anchor.y(),
            };

            (y + length, Polygon::rect(origin, size))
        }
        StemDirection::Down => {
            let nw_anchor = anchors
                .stem_down_nw
                .unwrap_or_else(|| panic!("{glyph:?} should have stem_down_nw anchor defined"));

            let origin = Coord {
                x: x + nw_anchor.x(),
                y: y - length,
            };

            let size = Size {
                width: stem_thickness,
                height: length + nw_anchor.y(),
            };

            (y - length, Polygon::rect(origin, size))
        }
    };

    (stem_end, Element::Polygon(stem_polygon))
}
