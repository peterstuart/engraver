use color_eyre::eyre::Result;
use engraver::render::input::{duration, measure, Accidental, Duration, Measure, Note, Staff};
use smufl::StaffSpaces;
use strum::IntoEnumIterator;

macro_rules! assert_notes_with_positions_snapshot {
    ($name:ident, $position:literal) => {
        #[test]
        fn $name() -> color_eyre::Result<()> {
            let elements = duration::Value::iter()
                .map(|value| {
                    measure::Element::Note(Note {
                        y: StaffSpaces($position),
                        accidental: None,
                        duration: Duration { value, dots: None },
                        beam: None,
                        id: None,
                    })
                })
                .collect();

            let staff = Staff {
                measures: vec![Measure {
                    elements,
                    ..Default::default()
                }],
                ..Default::default()
            };

            assert_staff_snapshot!(staff);

            Ok(())
        }
    };
}

assert_notes_with_positions_snapshot!(all_noteheads_stems_up, 0.0);
assert_notes_with_positions_snapshot!(all_noteheads_stems_down, 4.0);

assert_notes_with_positions_snapshot!(all_noteheads_leger_lines_above_on_line, 6.0);
assert_notes_with_positions_snapshot!(all_noteheads_leger_lines_above_on_space, 6.5);

assert_notes_with_positions_snapshot!(all_noteheads_leger_lines_below_on_line, -2.0);
assert_notes_with_positions_snapshot!(all_noteheads_leger_lines_below_on_space, -1.5);

#[test]
fn accidentals() -> Result<()> {
    let elements = Accidental::iter()
        .map(|accidental| {
            measure::Element::Note(Note {
                y: StaffSpaces(0.0),
                accidental: Some(accidental),
                duration: Duration {
                    value: duration::Value::Quarter,
                    dots: None,
                },
                beam: None,
                id: None,
            })
        })
        .collect();

    let staff = Staff {
        measures: vec![Measure {
            elements,
            ..Default::default()
        }],
        ..Default::default()
    };

    assert_staff_snapshot!(staff);

    Ok(())
}

#[test]
fn note_with_id() -> Result<()> {
    let staff = Staff {
        measures: vec![Measure {
            elements: vec![measure::Element::Note(Note {
                y: StaffSpaces(0.0),
                accidental: None,
                duration: Duration {
                    value: duration::Value::Whole,
                    dots: None,
                },
                beam: None,
                id: Some("test_note".to_string()),
            })],
            ..Default::default()
        }],
        ..Default::default()
    };

    assert_staff_snapshot!(staff);

    Ok(())
}
