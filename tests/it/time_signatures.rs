macro_rules! assert_time_signature_snapshot {
    ($name:ident, $numerator:literal, $denominator:literal) => {
        #[test]
        fn $name() -> color_eyre::Result<()> {
            let staff = engraver::render::input::Staff {
                time_signature: Some(engraver::render::input::TimeSignature {
                    numerator: $numerator,
                    denominator: $denominator,
                }),
                ..Default::default()
            };
            assert_staff_snapshot!(staff);

            Ok(())
        }
    };
}

assert_time_signature_snapshot!(even_top_and_bottom, 4, 4);
assert_time_signature_snapshot!(wide_on_top, 12, 8);
assert_time_signature_snapshot!(wide_on_bottom, 8, 12);
assert_time_signature_snapshot!(very_wide_on_top, 128, 1);
assert_time_signature_snapshot!(very_wide_on_bottom, 1, 128);
