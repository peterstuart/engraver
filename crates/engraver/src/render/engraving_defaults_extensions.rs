use smufl::{EngravingDefaults, StaffSpaces};

pub trait EngravingDefaultsExtensions {
    fn staff_line_thickness(&self) -> StaffSpaces;

    fn stem_thickness(&self) -> StaffSpaces;

    fn beam_thickness(&self) -> StaffSpaces;

    fn beam_spacing(&self) -> StaffSpaces;

    fn leger_line_thickness(&self) -> StaffSpaces;

    fn leger_line_extension(&self) -> StaffSpaces;

    fn slur_endpoint_thickness(&self) -> StaffSpaces;

    fn slur_midpoint_thickness(&self) -> StaffSpaces;

    fn tie_endpoint_thickness(&self) -> StaffSpaces;

    fn tie_midpoint_thickness(&self) -> StaffSpaces;

    fn thin_barline_thickness(&self) -> StaffSpaces;

    fn thick_barline_thickness(&self) -> StaffSpaces;

    fn dashed_barline_thickness(&self) -> StaffSpaces;

    fn dashed_barline_dash_length(&self) -> StaffSpaces;

    fn dashed_barline_gap_length(&self) -> StaffSpaces;

    fn barline_separation(&self) -> StaffSpaces;

    fn thin_thick_barline_separation(&self) -> StaffSpaces;

    fn repeat_barline_dot_separation(&self) -> StaffSpaces;

    fn bracket_thickness(&self) -> StaffSpaces;

    fn sub_bracket_thickness(&self) -> StaffSpaces;

    fn hairpin_thickness(&self) -> StaffSpaces;

    fn octave_line_thickness(&self) -> StaffSpaces;

    fn pedal_line_thickness(&self) -> StaffSpaces;

    fn repeat_ending_line_thickness(&self) -> StaffSpaces;

    fn arrow_shaft_thickness(&self) -> StaffSpaces;

    fn lyric_line_thickness(&self) -> StaffSpaces;

    fn text_enclosure_thickness(&self) -> StaffSpaces;

    fn tuplet_bracket_thickness(&self) -> StaffSpaces;

    fn h_bar_thickness(&self) -> StaffSpaces;
}

impl EngravingDefaultsExtensions for EngravingDefaults {
    fn staff_line_thickness(&self) -> StaffSpaces {
        self.staff_line_thickness.unwrap_or(StaffSpaces(0.1))
    }

    fn stem_thickness(&self) -> StaffSpaces {
        self.stem_thickness.unwrap_or(StaffSpaces(0.12))
    }

    fn beam_thickness(&self) -> StaffSpaces {
        self.beam_thickness.unwrap_or(StaffSpaces(0.5))
    }

    fn beam_spacing(&self) -> StaffSpaces {
        self.beam_spacing.unwrap_or(StaffSpaces(0.25))
    }

    fn leger_line_thickness(&self) -> StaffSpaces {
        self.leger_line_thickness.unwrap_or(StaffSpaces(0.16))
    }

    fn leger_line_extension(&self) -> StaffSpaces {
        self.leger_line_extension.unwrap_or(StaffSpaces(0.4))
    }

    fn slur_endpoint_thickness(&self) -> StaffSpaces {
        self.slur_endpoint_thickness.unwrap_or(StaffSpaces(0.1))
    }

    fn slur_midpoint_thickness(&self) -> StaffSpaces {
        self.slur_midpoint_thickness.unwrap_or(StaffSpaces(0.22))
    }

    fn tie_endpoint_thickness(&self) -> StaffSpaces {
        self.tie_endpoint_thickness.unwrap_or(StaffSpaces(0.1))
    }

    fn tie_midpoint_thickness(&self) -> StaffSpaces {
        self.tie_midpoint_thickness.unwrap_or(StaffSpaces(0.22))
    }

    fn thin_barline_thickness(&self) -> StaffSpaces {
        self.thin_barline_thickness.unwrap_or(StaffSpaces(0.16))
    }

    fn thick_barline_thickness(&self) -> StaffSpaces {
        self.thick_barline_thickness.unwrap_or(StaffSpaces(0.5))
    }

    fn dashed_barline_thickness(&self) -> StaffSpaces {
        self.dashed_barline_thickness.unwrap_or(StaffSpaces(0.16))
    }

    fn dashed_barline_dash_length(&self) -> StaffSpaces {
        self.dashed_barline_dash_length.unwrap_or(StaffSpaces(0.25))
    }

    fn dashed_barline_gap_length(&self) -> StaffSpaces {
        self.dashed_barline_gap_length.unwrap_or(StaffSpaces(0.25))
    }

    fn barline_separation(&self) -> StaffSpaces {
        self.barline_separation.unwrap_or(StaffSpaces(0.4))
    }

    /// This falls back to `barline_separation` if it is not present in the font
    /// metadata, as it is not currently supported by most fonts. See the
    /// [GitHub issue](https://github.com/w3c/smufl/issues/95) proposing this
    /// field for details.
    fn thin_thick_barline_separation(&self) -> StaffSpaces {
        self.thin_thick_barline_separation
            .unwrap_or_else(|| self.barline_separation())
    }

    fn repeat_barline_dot_separation(&self) -> StaffSpaces {
        self.repeat_barline_dot_separation
            .unwrap_or(StaffSpaces(0.16))
    }

    fn bracket_thickness(&self) -> StaffSpaces {
        self.bracket_thickness.unwrap_or(StaffSpaces(0.5))
    }

    fn sub_bracket_thickness(&self) -> StaffSpaces {
        self.sub_bracket_thickness.unwrap_or(StaffSpaces(0.16))
    }

    fn hairpin_thickness(&self) -> StaffSpaces {
        self.hairpin_thickness.unwrap_or(StaffSpaces(0.16))
    }

    fn octave_line_thickness(&self) -> StaffSpaces {
        self.octave_line_thickness.unwrap_or(StaffSpaces(0.16))
    }

    fn pedal_line_thickness(&self) -> StaffSpaces {
        self.pedal_line_thickness.unwrap_or(StaffSpaces(0.16))
    }

    fn repeat_ending_line_thickness(&self) -> StaffSpaces {
        self.repeat_ending_line_thickness
            .unwrap_or(StaffSpaces(0.16))
    }

    fn arrow_shaft_thickness(&self) -> StaffSpaces {
        self.arrow_shaft_thickness.unwrap_or(StaffSpaces(0.16))
    }

    fn lyric_line_thickness(&self) -> StaffSpaces {
        self.lyric_line_thickness.unwrap_or(StaffSpaces(0.16))
    }

    fn text_enclosure_thickness(&self) -> StaffSpaces {
        self.text_enclosure_thickness.unwrap_or(StaffSpaces(0.16))
    }

    fn tuplet_bracket_thickness(&self) -> StaffSpaces {
        self.tuplet_bracket_thickness.unwrap_or(StaffSpaces(0.16))
    }

    fn h_bar_thickness(&self) -> StaffSpaces {
        self.h_bar_thickness.unwrap_or(StaffSpaces(1.0))
    }
}
