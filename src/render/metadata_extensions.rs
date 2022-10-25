use smufl::{Glyph, Metadata, StaffSpaces};

pub trait MetadataExtensions {
    /// Returns a glyph's advance width, if one is provided, otherwise returns
    /// the x-coordinate of the right edge of the glyph's bounding box.
    fn width_of(&self, glyph: Glyph) -> StaffSpaces;
}

impl MetadataExtensions for Metadata {
    fn width_of(&self, glyph: Glyph) -> StaffSpaces {
        self.advance_widths
            .try_get(glyph)
            .unwrap_or_else(|| self.bounding_boxes.get(glyph).ne.x())
    }
}
