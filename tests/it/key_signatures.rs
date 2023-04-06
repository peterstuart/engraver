use color_eyre::eyre::Result;
use engraver::render::input::{key_signature, KeySignature, Staff};
use smufl::StaffSpaces;

fn staff_with_key_signature<Pitches>(kind: key_signature::Kind, pitches: Pitches) -> Staff
where
    Pitches: IntoIterator<Item = f64>,
{
    Staff {
        clef: None,
        key_signature: Some(KeySignature {
            kind,
            pitches: pitches.into_iter().map(StaffSpaces).collect(),
        }),
        time_signature: None,
        measures: vec![],
    }
}

#[test]
fn sharps() -> Result<()> {
    let staff = staff_with_key_signature(key_signature::Kind::Sharps, [4.0, 2.5, 4.5, 3.0]);
    assert_staff_snapshot!(staff);

    Ok(())
}

#[test]
fn flats() -> Result<()> {
    let staff = staff_with_key_signature(key_signature::Kind::Flats, [2.0, 3.5, 1.5, 3.0]);
    assert_staff_snapshot!(staff);

    Ok(())
}
