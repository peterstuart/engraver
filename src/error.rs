use smufl::Glyph;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing `{data_type}` data for glyph `{glyph:?}`")]
    MissingGlyphData { glyph: Glyph, data_type: String },

    #[error("Cannot start a beam when another beam is in progress")]
    StartedBeamWhileBeamInProgress,

    #[error("Cannot end a beam when no beam is in progress")]
    EndedBeamWhileNoBeamInProgress,
}

pub type Result<T> = std::result::Result<T, Error>;
