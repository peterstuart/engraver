use color_eyre::eyre::Result;
use engraver::{model, render::input};
use smufl::StaffSpaces;

#[test]
#[should_panic(expected = "chord must have at least 2 notes")]
fn input_panics_with_no_notes() {
    input::Chord::new(
        [],
        input::Duration {
            value: input::duration::Value::Whole,
            dots: None,
        },
        None,
        None,
    );
}

#[test]
#[should_panic(expected = "chord must have at least 2 notes")]
fn input_panics_with_one_note() {
    input::Chord::new(
        [input::chord::Note {
            y: StaffSpaces(0.0),
            accidental: None,
        }],
        input::Duration {
            value: input::duration::Value::Whole,
            dots: None,
        },
        None,
        None,
    );
}

#[test]
#[should_panic(expected = "chord must have at least 2 pitches")]
fn model_panics_with_no_notes() {
    model::Chord::new(
        [],
        model::Duration {
            value: model::duration::Value::Whole,
            dots: None,
        },
        None,
    );
}

#[test]
#[should_panic(expected = "chord must have at least 2 pitches")]
fn model_panics_with_one_note() {
    model::Chord::new(
        [model::Pitch {
            step: model::Step::C,
            alteration: model::Alteration::Natural,
            octave: 4,
        }],
        model::Duration {
            value: model::duration::Value::Whole,
            dots: None,
        },
        None,
    );
}

// See Gardner, p. 69:
// https://archive.org/details/musicnotationman00read/page/68
#[test]
fn stem_direction_gardner_example_5_12_1() {
    assert_staff_snapshot!("{g5 b} |");
}

#[test]
fn stem_direction_gardner_example_5_12_2() {
    assert_staff_snapshot!("{d5 a5 d6} |")
}

#[test]
fn stem_direction_gardner_example_5_12_3() {
    assert_staff_snapshot!("{b3 d4 f a} |")
}

#[test]
fn stem_direction_gardner_example_5_12_4() {
    assert_staff_snapshot!("{c e g c5} |")
}

// See Gardner, p. 71:
// https://archive.org/details/musicnotationman00read/page/70
#[test]
fn note_side_stem_up_left() {
    assert_staff_snapshot!("{e g b} |")
}

#[test]
fn note_side_stem_down_right() {
    assert_staff_snapshot!("{d5 f a} |")
}

#[test]
fn note_side_gardner_example_5_18_1() {
    assert_staff_snapshot!("{a b} |")
}

#[test]
fn note_side_gardner_example_5_18_3() {
    assert_staff_snapshot!("{e5 f} |")
}

#[test]
fn note_side_gardner_example_5_19_1() {
    assert_staff_snapshot!("{d f g b} |")
}

#[test]
fn note_side_gardner_example_5_19_3() {
    assert_staff_snapshot!("{b d5 f g} |")
}

#[test]
fn chord_with_id() -> Result<()> {
    let staff = input::Staff {
        measures: vec![input::measure::Measure {
            elements: vec![input::measure::Element::Chord(input::Chord::new(
                [
                    input::chord::Note {
                        y: StaffSpaces(0.0),
                        accidental: None,
                    },
                    input::chord::Note {
                        y: StaffSpaces(1.0),
                        accidental: None,
                    },
                    input::chord::Note {
                        y: StaffSpaces(2.0),
                        accidental: None,
                    },
                ],
                input::Duration {
                    value: input::duration::Value::Whole,
                    dots: None,
                },
                None,
                Some("test_chord".to_string()),
            ))],
            ..Default::default()
        }],
        ..Default::default()
    };

    assert_staff_snapshot!(staff);

    Ok(())
}
