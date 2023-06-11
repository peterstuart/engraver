use super::{Context, Duration, Pitch};
use crate::render;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: Duration,
    pub id: Option<String>,
}

impl Note {
    pub(crate) fn into_input(self, context: &mut Context) -> render::input::Note {
        let (y, accidental) = self.pitch.into_input(context);

        render::input::Note {
            y,
            accidental,
            duration: self.duration,
            beam: None,
            id: self.id,
        }
    }
}
