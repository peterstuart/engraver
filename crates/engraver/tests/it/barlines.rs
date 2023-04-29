use color_eyre::eyre::Result;
use engraver::render::input::{Barline, Measure, Staff};
use strum::IntoEnumIterator;

#[test]
fn all() -> Result<()> {
    let measures = Barline::iter()
        .map(|barline| Measure {
            elements: vec![],
            barline,
        })
        .collect();
    let staff = Staff {
        measures,
        ..Default::default()
    };

    assert_staff_snapshot!(staff);

    Ok(())
}
