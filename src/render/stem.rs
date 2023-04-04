use smufl::{Glyph, Metadata, StaffSpaces};

use super::{
    engraving_defaults_extensions::EngravingDefaultsExtensions,
    glyph_data_extensions::GlyphDataExtensions,
    ir::{Coord, Element, Polygon, Size},
};
use crate::{Error, Result};

pub const DEFAULT_LENGTH: StaffSpaces = StaffSpaces(3.5);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stem {
    glyph: Glyph,
    x: StaffSpaces,
    y: StaffSpaces,
    direction: Direction,
    length: StaffSpaces,
}

impl Stem {
    pub fn new(
        glyph: Glyph,
        x: StaffSpaces,
        y: StaffSpaces,
        direction: Direction,
        length: Option<StaffSpaces>,
    ) -> Self {
        Self {
            glyph,
            x,
            y,
            direction,
            length: length.unwrap_or(DEFAULT_LENGTH),
        }
    }

    pub fn end(&self) -> StaffSpaces {
        match self.direction {
            Direction::Up => self.y + self.length,
            Direction::Down => self.y - self.length,
        }
    }

    pub fn left(&self, metadata: &Metadata) -> Result<StaffSpaces> {
        let anchor = self.anchor(metadata)?;

        Ok(match self.direction {
            Direction::Up => {
                let stem_thickness = metadata.engraving_defaults.stem_thickness();
                self.x + anchor.x() - stem_thickness
            }
            Direction::Down => self.x + anchor.x(),
        })
    }

    pub fn right(&self, metadata: &Metadata) -> Result<StaffSpaces> {
        let stem_thickness = metadata.engraving_defaults.stem_thickness();
        Ok(self.left(metadata)? + stem_thickness)
    }

    pub fn adjust_length(&mut self, adjustment: StaffSpaces) {
        self.length += adjustment;
    }

    pub fn render(&self, metadata: &Metadata) -> Result<Element<StaffSpaces>> {
        let stem_thickness = metadata.engraving_defaults.stem_thickness();
        let anchors = metadata.anchors.try_get(self.glyph)?;

        let polygon = match self.direction {
            Direction::Up => {
                let se_anchor = anchors.stem_up_se.ok_or(Error::MissingGlyphData {
                    glyph: self.glyph,
                    data_type: "Anchors.stem_up_se".to_owned(),
                })?;

                let origin = Coord {
                    x: self.left(metadata)?,
                    y: self.y + se_anchor.y(),
                };

                let size = Size {
                    width: stem_thickness,
                    height: self.length - se_anchor.y(),
                };

                Polygon::rect(origin, size)
            }
            Direction::Down => {
                let nw_anchor = anchors.stem_down_nw.ok_or(Error::MissingGlyphData {
                    glyph: self.glyph,
                    data_type: "Anchors.stem_down_nw".to_owned(),
                })?;

                let origin = Coord {
                    x: self.left(metadata)?,
                    y: self.y - self.length,
                };

                let size = Size {
                    width: stem_thickness,
                    height: self.length + nw_anchor.y(),
                };

                Polygon::rect(origin, size)
            }
        };

        Ok(Element::Polygon(polygon))
    }

    fn anchor(&self, metadata: &Metadata) -> Result<smufl::Coord> {
        let anchors = metadata.anchors.try_get(self.glyph)?;

        match self.direction {
            Direction::Up => anchors.stem_up_se.ok_or(Error::MissingGlyphData {
                glyph: self.glyph,
                data_type: "Anchors.stem_up_se".to_owned(),
            }),
            Direction::Down => anchors.stem_down_nw.ok_or(Error::MissingGlyphData {
                glyph: self.glyph,
                data_type: "Anchors.stem_down_nw".to_owned(),
            }),
        }
    }
}
