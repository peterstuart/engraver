use smufl::StaffSpaces;

use super::{
    duration,
    note::{create_accidental, create_flag, create_leger_lines, create_notehead},
    Accidental, Beam, Duration,
};
use crate::{
    render::{
        context::{beam, Context},
        ir::Group,
        metadata_extensions::MetadataExtensions,
        stem::{self, Stem},
        Element, Output, Render,
    },
    Result,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Chord {
    notes: Vec<Note>,
    pub duration: Duration,
    pub beam: Option<Beam>,
    pub id: Option<String>,
}

/// Which side of a stem a chord notehead should be drawn on.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Side {
    /// The default side of the stem.
    ///
    /// (ie. left for [up](stem::Direction::Up), right for
    /// [down](stem::Direction::Down))
    Default,

    /// The side opposite the default side.
    ///
    /// (ie. right for [up][stem::Direction::Up] left for
    /// [down](stem::Direction::Down))
    Opposite,
}

impl Chord {
    /// Returns a `Chord` with the given `notes` and `duration`.
    ///
    /// # Panics
    ///
    /// The function will panic if `notes` does not contain at least 2 notes.
    pub fn new<Notes>(
        notes: Notes,
        duration: Duration,
        beam: Option<Beam>,
        id: Option<String>,
    ) -> Self
    where
        Notes: IntoIterator<Item = Note>,
    {
        let mut notes = notes.into_iter().collect::<Vec<_>>();

        assert!(notes.len() > 1, "chord must have at least 2 notes");

        notes.sort_by(|note1, note2| {
            (note1.y, note1.accidental)
                .partial_cmp(&(note2.y, note2.accidental))
                .expect("notes should always be orderable")
        });

        Self {
            notes,
            duration,
            beam,
            id,
        }
    }

    /// Returns the preferred stem direction for the chord, if there is one.
    ///
    /// If `None` is returned, either direction is acceptable.
    ///
    /// > The direction of the stem has to be determined by the general position
    /// > of the ... chord on the staff. If most of the notes lie at the *top*
    /// > of the staff, the stem will ordinarily go *down*. If the chord is
    /// > positioned on the *bottom* part of the staff, the stem usually goes
    /// > *up*. When chord notes are both high and low on the staff, the
    /// > direction of the stem has to be determined by the *available room*
    /// > above or below the staff. Common sense should determine which
    /// > direction is best for visual clarity.
    ///
    /// [Gardner, p. 69](https://archive.org/details/musicnotationman00read/page/69)
    fn stem_direction(&self) -> Option<stem::Direction> {
        const MIDDLE_STAFF_LINE: StaffSpaces = StaffSpaces(2.0);

        let lowest = self.lowest_note().y;
        let highest = self.highest_note().y;

        let lowest_distance_to_middle = MIDDLE_STAFF_LINE - lowest;
        let highest_distance_to_middle = highest - MIDDLE_STAFF_LINE;

        if lowest_distance_to_middle == highest_distance_to_middle {
            None
        } else if lowest_distance_to_middle > highest_distance_to_middle {
            Some(stem::Direction::Up)
        } else {
            Some(stem::Direction::Down)
        }
    }

    /// Returns the notes of the chord, paired with the side of the stem on
    /// which they should appear, ordered from the start of the stem to the end
    /// of the stem.
    ///
    /// > The interval of a *second* (the two notes on adjacent scale-steps)
    /// > should be written with the stem *between* the note-heads. The higher
    /// > pitch is always placed to the right, never to the left, regardless of
    /// > stem direction.
    ///
    /// [Gardner, p. 71](https://archive.org/details/musicnotationman00read/page/71)
    fn notes(&self, stem_direction: stem::Direction) -> Vec<(Note, Side)> {
        let mut notes_with_sides = Vec::with_capacity(self.notes.len());

        match stem_direction {
            stem::Direction::Up => {
                for (index, note) in self.notes.iter().copied().enumerate() {
                    if index == 0 {
                        notes_with_sides.push((note, Side::Default));
                        continue;
                    }

                    let (prev_note, prev_side) = notes_with_sides[index - 1];

                    let side =
                        if (note.y - prev_note.y).0.abs() <= 0.5 && prev_side == Side::Default {
                            Side::Opposite
                        } else {
                            Side::Default
                        };

                    notes_with_sides.push((note, side));
                }
            }
            stem::Direction::Down => {
                for (index, note) in self.notes.iter().copied().enumerate() {
                    if index == self.notes.len() - 1 {
                        notes_with_sides.push((note, Side::Default));
                        continue;
                    }

                    let next_note = self.notes[index + 1];

                    let side = if (next_note.y - note.y).0.abs() <= 0.5 {
                        Side::Opposite
                    } else {
                        Side::Default
                    };

                    notes_with_sides.push((note, side));
                }

                notes_with_sides.reverse();
            }
        };

        notes_with_sides
    }

    /// Returns the lowest note in the chord.
    fn lowest_note(&self) -> Note {
        *self.notes.first().unwrap()
    }

    /// Returns the highest note in the chord.
    fn highest_note(&self) -> Note {
        *self.notes.last().unwrap()
    }
}

impl Render for Chord {
    fn render(
        &self,
        x: StaffSpaces,
        context: &mut Context,
        metadata: &smufl::Metadata,
    ) -> Result<Output> {
        let glyph = self.duration.value.notehead_glyph();
        let width = metadata.width_of(glyph)?;

        let stem_direction = context
            .beam()
            .map(|beam| beam.stem_direction)
            .or_else(|| self.stem_direction())
            .unwrap_or(stem::Direction::Up);

        let notes = self.notes(stem_direction);

        let mut elements = notes
            .iter()
            .map(|(note, side)| {
                let x = match (side, stem_direction) {
                    (Side::Default, _) => x,
                    (Side::Opposite, stem::Direction::Up) => x + width,
                    (Side::Opposite, stem::Direction::Down) => x - width,
                };

                create_notehead(x, note.y, glyph)
            })
            .collect::<Vec<_>>();

        let mut leger_lines = create_leger_lines(x, notes.first().unwrap().0.y, glyph, metadata)?;
        elements.append(&mut leger_lines);

        let mut leger_lines = create_leger_lines(x, notes.last().unwrap().0.y, glyph, metadata)?;
        elements.append(&mut leger_lines);

        let mut accidentals = notes
            .iter()
            .filter_map(|(note, _side)| {
                note.accidental.map(|accidental| {
                    let glyph = accidental.glyph();
                    create_accidental(x, note.y, glyph, metadata)
                })
            })
            .collect::<Result<Vec<_>>>()?;
        elements.append(&mut accidentals);

        let start_y = match stem_direction {
            stem::Direction::Up => self.lowest_note(),
            stem::Direction::Down => self.highest_note(),
        }
        .y;

        let length = self.highest_note().y - self.lowest_note().y;

        match context.beam() {
            Some(beam) => beam.add_notehead(beam::Notehead {
                glyph,
                x,
                y: start_y,
                min_stem_length: Some(length),
            }),
            None => {
                if self.duration.value != duration::Value::Whole {
                    let stem = Stem::new(glyph, x, start_y, stem_direction, Some(length));

                    if let Some(flag_glyph) = self.duration.value.flag_glyph(stem_direction) {
                        let flag = create_flag(
                            x,
                            stem.end(),
                            glyph,
                            flag_glyph,
                            stem_direction,
                            metadata,
                        )?;
                        elements.push(flag);
                    }

                    elements.push(stem.render(metadata)?);
                }
            }
        };

        if self.id.is_some() {
            Ok(Output {
                elements: vec![Element::Group(Group {
                    id: self.id.clone(),
                    elements,
                })],
                width,
            })
        } else {
            Ok(Output { elements, width })
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Note {
    pub y: StaffSpaces,
    pub accidental: Option<Accidental>,
}
