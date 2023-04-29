use std::any::type_name;

use smufl::{Glyph, GlyphData};

use crate::{Error, Result};

pub trait GlyphDataExtensions {
    type Value;

    fn try_get(&self, glyph: Glyph) -> Result<Self::Value>;
}

impl<T> GlyphDataExtensions for GlyphData<T>
where
    T: Copy,
{
    type Value = T;

    fn try_get(&self, glyph: Glyph) -> Result<T> {
        self.get(glyph).ok_or_else(|| Error::MissingGlyphData {
            glyph,
            data_type: type_name::<T>().to_owned(),
        })
    }
}
