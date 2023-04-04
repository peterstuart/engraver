use color_eyre::eyre::Result;
use engraver::render::input::{duration, measure, Accidental, Duration, Measure, Note, Staff};
use smufl::StaffSpaces;
use strum::IntoEnumIterator;

#[test]
fn all() -> Result<()> {
    let elements = duration::Value::iter()
        .map(|value| {
            measure::Element::Note(Note {
                y: StaffSpaces(0.0),
                accidental: None,
                duration: Duration { value, dots: None },
                beam: None,
            })
        })
        .collect();

    assert_single_measure_staff_snapshot!(elements);

    Ok(())
}

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
            })
        })
        .collect();

    assert_single_measure_staff_snapshot!(elements);

    Ok(())
}
