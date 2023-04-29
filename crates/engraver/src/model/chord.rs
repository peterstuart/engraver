use super::{Context, Duration, Pitch};
use crate::render;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Chord {
    pub pitches: Vec<Pitch>,
    pub duration: Duration,
}

impl Chord {
    pub fn new<Pitches>(pitches: Pitches, duration: Duration) -> Self
    where
        Pitches: IntoIterator<Item = Pitch>,
    {
        let pitches = pitches.into_iter().collect::<Vec<_>>();
        assert!(pitches.len() > 1, "chord must have at least 2 pitches");

        Self { pitches, duration }
    }

    pub(crate) fn into_input(self, context: &mut Context) -> render::input::Chord {
        let notes = self.pitches.into_iter().map(|pitch| {
            let (y, accidental) = pitch.into_input(context);
            render::input::chord::Note { y, accidental }
        });

        render::input::Chord::new(notes, self.duration, None)
    }
}
