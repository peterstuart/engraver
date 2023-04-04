use smufl::{Metadata, StaffSpaces};

use crate::{
    render::{
        context::Context,
        engraving_defaults_extensions::EngravingDefaultsExtensions,
        ir::{Coord, Element, Line, Linecap},
        Output, Render,
    },
    Result,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Barline {
    Thin,
    Final,
}

impl Default for Barline {
    fn default() -> Self {
        Self::Thin
    }
}

impl Render for Barline {
    fn render(
        &self,
        x: StaffSpaces,
        _context: &mut Context,
        metadata: &Metadata,
    ) -> Result<Output> {
        Ok(match self {
            Self::Thin => {
                let thickness = metadata.engraving_defaults.thin_barline_thickness();
                let x = x + thickness / 2.0;

                let line = Element::Line(Line {
                    from: Coord {
                        x,
                        y: StaffSpaces::zero(),
                    },
                    to: Coord {
                        x,
                        y: StaffSpaces(4.0),
                    },
                    thickness,
                    cap: Linecap::Butt,
                });

                Output {
                    elements: vec![line],
                    width: thickness,
                }
            }
            Self::Final => {
                let thin_thickness = metadata.engraving_defaults.thin_barline_thickness();
                let thin_x = x + thin_thickness / 2.0;

                let thin_line = Element::Line(Line {
                    from: Coord {
                        x: thin_x,
                        y: StaffSpaces::zero(),
                    },
                    to: Coord {
                        x: thin_x,
                        y: StaffSpaces(4.0),
                    },
                    thickness: thin_thickness,
                    cap: Linecap::Butt,
                });

                let separation = metadata.engraving_defaults.thin_thick_barline_separation();

                let thick_thickness = metadata.engraving_defaults.thick_barline_thickness();
                let thick_x = x + thin_thickness + separation + thick_thickness / 2.0;

                let thick_line = Element::Line(Line {
                    from: Coord {
                        x: thick_x,
                        y: StaffSpaces::zero(),
                    },
                    to: Coord {
                        x: thick_x,
                        y: StaffSpaces(4.0),
                    },
                    thickness: thick_thickness,
                    cap: Linecap::Butt,
                });

                Output {
                    elements: vec![thin_line, thick_line],
                    width: thin_thickness + separation + thick_thickness,
                }
            }
        })
    }
}
