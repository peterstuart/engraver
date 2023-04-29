use smufl::{Glyph, Metadata, StaffSpaces};

use super::glyph_data_extensions::GlyphDataExtensions;
use crate::Result;

pub trait MetadataExtensions {
    /// Returns a glyph's advance width, if one is provided, otherwise returns
    /// the x-coordinate of the right edge of the glyph's bounding box.
    fn width_of(&self, glyph: Glyph) -> Result<StaffSpaces>;
}

impl MetadataExtensions for Metadata {
    fn width_of(&self, glyph: Glyph) -> Result<StaffSpaces> {
        self.advance_widths.try_get(glyph).or_else(|_| {
            self.bounding_boxes
                .try_get(glyph)
                .map(|bounding_box| bounding_box.ne.x())
        })
    }
}
