#[test]
fn treble_clef_sharps() {
    assert_staff_snapshot!("####### |");
}

#[test]
fn treble_clef_flats() {
    assert_staff_snapshot!("bbbbbbb |");
}

#[test]
fn alto_clef_sharps() {
    assert_staff_snapshot!("alto ####### |");
}

#[test]
fn alto_clef_flats() {
    assert_staff_snapshot!("alto bbbbbbb |");
}

#[test]
fn tenor_clef_sharps() {
    assert_staff_snapshot!("tenor ####### |");
}

#[test]
fn tenor_clef_flats() {
    assert_staff_snapshot!("tenor bbbbbbb |");
}

#[test]
fn bass_clef_sharps() {
    assert_staff_snapshot!("bass ####### |");
}

#[test]
fn bass_clef_flats() {
    assert_staff_snapshot!("bass bbbbbbb |");
}
