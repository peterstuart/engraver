pub mod beam;

pub use beam::Beam;

use super::stem;
use crate::{Error, Result};

#[derive(Debug, Default)]
pub struct Context {
    beam: Option<Beam>,
}

impl Context {
    pub fn begin_beam(&mut self, stem_direction: stem::Direction) -> Result<()> {
        match self.beam {
            Some(_) => Err(Error::StartedBeamWhileBeamInProgress),
            None => {
                self.beam = Some(Beam::new(stem_direction));
                Ok(())
            }
        }
    }

    pub fn end_beam(&mut self) -> Result<Beam> {
        self.beam
            .take()
            .ok_or(Error::EndedBeamWhileNoBeamInProgress)
    }

    pub fn beam(&mut self) -> Option<&mut Beam> {
        self.beam.as_mut()
    }
}
