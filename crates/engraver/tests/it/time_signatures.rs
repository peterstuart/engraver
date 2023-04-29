#[test]
fn even_top_and_bottom() {
    assert_staff_snapshot!("4/4");
}

#[test]
fn wide_on_top() {
    assert_staff_snapshot!("12/8");
}

#[test]
fn wide_on_bottom() {
    assert_staff_snapshot!("8/12");
}

#[test]
fn very_wide_on_top() {
    assert_staff_snapshot!("128/1");
}

#[test]
fn very_wide_on_bottom() {
    assert_staff_snapshot!("1/128");
}
