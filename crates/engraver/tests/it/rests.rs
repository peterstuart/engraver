use color_eyre::eyre::Result;
use engraver::render::input::{duration, measure, Duration, Measure, Rest, Staff};
use strum::IntoEnumIterator;

#[test]
fn all() -> Result<()> {
    let elements = duration::Value::iter()
        .map(|value| {
            measure::Element::Rest(Rest {
                duration: Duration { value, dots: None },
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
fn rest_with_id() -> Result<()> {
    let staff = Staff {
        measures: vec![Measure {
            elements: vec![measure::Element::Rest(Rest {
                duration: Duration {
                    value: duration::Value::Whole,
                    dots: None,
                },
                id: Some("test_rest".to_string()),
            })],
            ..Default::default()
        }],
        ..Default::default()
    };

    assert_staff_snapshot!(staff);

    Ok(())
}
