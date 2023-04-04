use color_eyre::eyre::Result;
use engraver::render::input::{duration, measure, Duration, Measure, Rest, Staff};
use strum::IntoEnumIterator;

#[test]
fn all() -> Result<()> {
    let elements = duration::Value::iter()
        .map(|value| {
            measure::Element::Rest(Rest {
                duration: Duration { value, dots: None },
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
