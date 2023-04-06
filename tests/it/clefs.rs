use smufl::Glyph;

macro_rules! assert_clef_snapshot {
    ($name:ident, $glyph:expr, $y:literal) => {
        #[test]
        fn $name() -> color_eyre::Result<()> {
            let staff = engraver::render::input::Staff {
                clef: Some(engraver::render::input::Clef {
                    glyph: $glyph,
                    y: smufl::StaffSpaces($y),
                }),
                ..Default::default()
            };
            assert_staff_snapshot!(staff);

            Ok(())
        }
    };
}

assert_clef_snapshot!(g_clef, Glyph::GClef, 1.0);
assert_clef_snapshot!(f_clef, Glyph::FClef, 3.0);
assert_clef_snapshot!(c_clef, Glyph::CClef, 2.0);
