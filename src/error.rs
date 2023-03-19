use smufl::Glyph;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing `{data_type}` data for glyph `{glyph:?}`")]
    MissingGlyphData { glyph: Glyph, data_type: String },
}

pub type Result<T> = std::result::Result<T, Error>;
