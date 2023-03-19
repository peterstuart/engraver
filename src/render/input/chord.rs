use smufl::StaffSpaces;

use super::{
    duration,
    note::{
        create_accidental, create_flag, create_leger_lines, create_notehead, create_stem,
        DEFAULT_STEM_LENGTH,
    },
    Accidental, Duration,
};
use crate::{
    render::{metadata_extensions::MetadataExtensions, Output, Render, StemDirection},
    Result,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Chord {
    notes: Vec<Note>,
    pub duration: Duration,
}

/// Which side of a stem a chord notehead should be drawn on.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Side {
    /// The default side of the stem.
    ///
    /// (ie. left for [up](StemDirection::Up), right for
    /// [down](StemDirection::Down))
    Default,

    /// The side opposite the default side.
    ///
    /// (ie. right for [up][StemDirection::Up] left for
    /// [down](StemDirection::Down))
    Opposite,
}

impl Chord {
    /// Returns a `Chord` with the given `notes` and `duration`.
    ///
    /// # Panics
    ///
    /// The function will panic if `notes` does not contain at least 2 notes.
    pub fn new<Notes>(notes: Notes, duration: Duration) -> Self
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

        Self { notes, duration }
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
    fn stem_direction(&self) -> Option<StemDirection> {
        const MIDDLE_STAFF_LINE: StaffSpaces = StaffSpaces(2.0);

        let lowest = self.lowest_note().y;
        let highest = self.highest_note().y;

        let lowest_distance_to_middle = MIDDLE_STAFF_LINE - lowest;
        let highest_distance_to_middle = highest - MIDDLE_STAFF_LINE;

        if lowest_distance_to_middle == highest_distance_to_middle {
            None
        } else if lowest_distance_to_middle > highest_distance_to_middle {
            Some(StemDirection::Up)
        } else {
            Some(StemDirection::Down)
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
    fn notes(&self, stem_direction: StemDirection) -> Vec<(Note, Side)> {
        let mut notes_with_sides = Vec::with_capacity(self.notes.len());

        match stem_direction {
            StemDirection::Up => {
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
            StemDirection::Down => {
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
    fn render(&self, x: StaffSpaces, metadata: &smufl::Metadata) -> Result<Output> {
        let glyph = self.duration.value.notehead_glyph();
        let width = metadata.width_of(glyph)?;

        let stem_direction = self.stem_direction().unwrap_or(StemDirection::Up);

        let notes = self.notes(stem_direction);

        let mut elements = notes
            .iter()
            .map(|(note, side)| {
                let x = match (side, stem_direction) {
                    (Side::Default, _) => x,
                    (Side::Opposite, StemDirection::Up) => x + width,
                    (Side::Opposite, StemDirection::Down) => x - width,
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

        if self.duration.value != duration::Value::Whole {
            let start_y = match stem_direction {
                StemDirection::Up => self.lowest_note().y,
                StemDirection::Down => self.highest_note().y,
            };

            let length = self.highest_note().y - self.lowest_note().y + DEFAULT_STEM_LENGTH;

            let (stem_end, stem) =
                create_stem(x, start_y, length, stem_direction, glyph, metadata)?;

            if let Some(flag_glyph) = self.duration.value.flag_glyph(stem_direction) {
                let flag = create_flag(x, stem_end, glyph, flag_glyph, stem_direction, metadata)?;
                elements.push(flag);
            }

            elements.push(stem);
        }

        Ok(Output { elements, width })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Note {
    pub y: StaffSpaces,
    pub accidental: Option<Accidental>,
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;
    use crate::render::input::duration;

    /// Make a chord with notes at the provided positions on the staff.
    fn make_chord<Positions>(positions: Positions) -> Chord
    where
        Positions: IntoIterator<Item = f64>,
    {
        Chord::new(
            make_notes(positions),
            Duration {
                value: duration::Value::Half,
                dots: None,
            },
        )
    }

    fn make_notes<Positions>(positions: Positions) -> Vec<Note>
    where
        Positions: IntoIterator<Item = f64>,
    {
        positions
            .into_iter()
            .map(|y| Note {
                y: StaffSpaces(y),
                accidental: None,
            })
            .collect()
    }

    fn make_notes_with_sides<Positions>(positions: Positions) -> Vec<(Note, Side)>
    where
        Positions: IntoIterator<Item = (f64, Side)>,
    {
        positions
            .into_iter()
            .map(|(y, side)| {
                (
                    Note {
                        y: StaffSpaces(y),
                        accidental: None,
                    },
                    side,
                )
            })
            .collect()
    }

    #[rstest]
    #[should_panic]
    #[case::no_notes(make_notes([]))]
    #[should_panic]
    #[case::one_note(make_notes([0.0]))]
    #[case::two_notes(make_notes([0.0,1.0]))]
    fn new(#[case] notes: Vec<Note>) {
        Chord::new(
            notes,
            Duration {
                value: duration::Value::Whole,
                dots: None,
            },
        );
    }

    // See Gardner, p. 69:
    // https://archive.org/details/musicnotationman00read/page/68
    #[rstest]
    #[case::gardner_example_5_12_1(make_chord([4.5, 5.5]), Some(StemDirection::Down))]
    #[case::gardner_example_5_12_2(make_chord([3.0, 5.0, 6.5]), Some(StemDirection::Down))]
    #[case::gardner_example_5_12_3(make_chord([-1.5, -0.5, 0.5, 1.5]), Some(StemDirection::Up))]
    #[case::gardner_example_5_12_4(make_chord([-1.0, 0.0, 1.0, 2.5]), Some(StemDirection::Up))]
    #[case::gardner_example_5_12_5(make_chord([1.0, 2.0, 3.0]), None)]
    #[case::gardner_example_5_12_6(make_chord([-0.5, 2.0, 4.5]), None)]
    fn stem_direction(
        #[case] chord: Chord,
        #[case] expected_stem_direction: Option<StemDirection>,
    ) {
        assert_eq!(chord.stem_direction(), expected_stem_direction);
    }

    // See Gardner, p. 70:
    // https://archive.org/details/musicnotationman00read/page/70
    #[rstest]
    #[case(
        make_chord([0.0, 1.0, 2.0]),
        StemDirection::Up,
        make_notes_with_sides([(0.0, Side::Default), (1.0, Side::Default), (2.0, Side::Default)])
    )]
    #[case(
        make_chord([0.0, 1.0, 2.0]),
        StemDirection::Down,
        make_notes_with_sides([(2.0, Side::Default), (1.0, Side::Default), (0.0, Side::Default)])
    )]
    #[case(
        make_chord([2.0, 1.0, 0.0]),
        StemDirection::Up,
        make_notes_with_sides([(0.0, Side::Default), (1.0, Side::Default), (2.0, Side::Default)])
    )]
    #[case(
        make_chord([2.0, 1.0, 0.0]),
        StemDirection::Down,
        make_notes_with_sides([(2.0, Side::Default), (1.0, Side::Default), (0.0, Side::Default)])
    )]
    #[case::gardner_example_5_18_1(
        make_chord([1.5, 2.0]),
        StemDirection::Up,
        make_notes_with_sides([(1.5, Side::Default), (2.0, Side::Opposite)])
    )]
    #[case::gardner_example_5_18_3(
        make_chord([3.5, 4.0]),
        StemDirection::Down,
        make_notes_with_sides([(4.0, Side::Default), (3.5, Side::Opposite)])
    )]
    #[case::gardner_example_5_19_1(
        make_chord([-0.5, 0.5, 1.0, 2.0]),
        StemDirection::Up,
        make_notes_with_sides([(-0.5, Side::Default), (0.5, Side::Default), (1.0, Side::Opposite), (2.0, Side::Default)])
    )]
    #[case::gardner_example_5_19_3(
        make_chord([2.0, 3.0, 4.0, 4.5]),
        StemDirection::Down,
        make_notes_with_sides([(4.5, Side::Default), (4.0, Side::Opposite), (3.0, Side::Default), (2.0, Side::Default)])
    )]
    fn notes(
        #[case] chord: Chord,
        #[case] stem_direction: StemDirection,
        #[case] expected_notes: Vec<(Note, Side)>,
    ) {
        assert_eq!(chord.notes(stem_direction), expected_notes);
    }
}
