use color_eyre::eyre::Result;

#[test]
fn no_accidental_when_no_alteration_in_key_with_no_sharps() {
    assert_staff_snapshot!("f |");
}

#[test]
fn no_accidental_when_sharp_in_key_with_sharps() {
    assert_staff_snapshot!("### f# |");
}

#[test]
fn accidental_when_natural_in_key_with_sharps() {
    assert_staff_snapshot!("### f |");
}

#[test]
fn accidental_when_flat_in_key_with_sharps() {
    assert_staff_snapshot!("### fb |");
}

#[test]
fn accidental_when_alteration_not_in_key() {
    assert_staff_snapshot!("f# |");
}

#[test]
fn no_accidental_when_alteration_repeated() {
    assert_staff_snapshot!("f# f# |");
}

#[test]
fn accidental_when_alteration_repeated_over_barline() {
    assert_staff_snapshot!("f# | f# |");
}

#[test]
fn accidental_canceled_in_next_bar() {
    assert_staff_snapshot!("f# | f |");
}

#[test]
fn key_signature_alteration() -> Result<()> {
    assert_staff_snapshot!("c d# e f | c d# e# f ||");

    Ok(())
}
