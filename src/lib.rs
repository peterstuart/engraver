#![deny(rustdoc::broken_intra_doc_links)]

pub mod render;
pub mod svg;

mod error;

pub use error::{Error, Result};
