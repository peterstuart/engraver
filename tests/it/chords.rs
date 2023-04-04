use engraver::render::input::{chord::Note, duration, measure, Chord, Duration, Measure, Staff};
use rstest::*;
use smufl::StaffSpaces;

macro_rules! assert_chord_snapshot {
    ($name:ident, $position:literal, $($more:literal),+) => {
        #[test]
        fn $name() -> color_eyre::Result<()> {
            let staff = make_staff([$position, $( $more ),*]);
            assert_staff_snapshot!(staff);

            Ok(())
        }
    };
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

fn make_staff<Positions>(positions: Positions) -> Staff
where
    Positions: IntoIterator<Item = f64>,
{
    let elements = vec![measure::Element::Chord(Chord::new(
        make_notes(positions),
        Duration {
            value: duration::Value::Quarter,
            dots: None,
        },
        None,
    ))];

    engraver::render::input::Staff {
        measures: vec![Measure {
            elements,
            ..Default::default()
        }],
        ..Default::default()
    }
}

#[rstest]
// panics with no notes
#[should_panic]
#[case::no_notes(make_notes([]))]
// panics with 1 note
#[should_panic]
#[case::one_note(make_notes([0.0]))]
// does not panic with >= 2 notes
#[case::two_notes(make_notes([0.0, 1.0]))]
fn new(#[case] notes: Vec<Note>) {
    Chord::new(
        notes,
        Duration {
            value: duration::Value::Whole,
            dots: None,
        },
        None,
    );
}

// See Gardner, p. 69:
// https://archive.org/details/musicnotationman00read/page/68
assert_chord_snapshot!(stem_direction_gardner_example_5_12_1, 4.5, 5.5);
assert_chord_snapshot!(stem_direction_gardner_example_5_12_2, 3.0, 5.0, 6.5);
assert_chord_snapshot!(stem_direction_gardner_example_5_12_3, -1.5, -0.5, 0.5, 1.5);
assert_chord_snapshot!(stem_direction_gardner_example_5_12_4, -1.0, 0.0, 1.0, 2.5);

// See Gardner, p. 71:
// https://archive.org/details/musicnotationman00read/page/70
assert_chord_snapshot!(note_side_stem_up_left, 0.0, 1.0, 2.0);
assert_chord_snapshot!(note_side_stem_down_right, 3.0, 4.0, 5.0);
assert_chord_snapshot!(note_side_gardner_example_5_18_1, 1.5, 2.0);
assert_chord_snapshot!(note_side_gardner_example_5_18_3, 3.5, 4.0);
assert_chord_snapshot!(note_side_gardner_example_5_19_1, -0.5, 0.5, 1.0, 2.0);
assert_chord_snapshot!(note_side_gardner_example_5_19_3, 2.0, 3.0, 4.0, 4.5);
