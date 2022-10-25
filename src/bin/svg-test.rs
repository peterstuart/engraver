use std::{
    env,
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use engraver::{
    render::input::{
        duration, key_signature, measure, Barline, Clef, Duration, KeySignature, Measure, Note,
        Rest, Staff, TimeSignature,
    },
    svg::{elements_to_svg_document, Options},
};
use smufl::{Glyph, StaffSpaces};

fn main() {
    let cargo_manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set");
    let root_path = Path::new(&cargo_manifest_dir);

    let mut font_metadata_path = root_path.to_path_buf();
    font_metadata_path.push("submodules/bravura/redist/bravura_metadata.json");

    let file = File::open(font_metadata_path).unwrap();
    let reader = BufReader::new(file);
    let metadata = smufl::Metadata::from_reader(reader).unwrap();

    let staff = Staff {
        clef: Some(Clef {
            glyph: Glyph::GClef,
            y: StaffSpaces(1.0),
        }),
        key_signature: Some(KeySignature {
            kind: key_signature::Kind::Sharps,
            pitches: vec![StaffSpaces(4.0), StaffSpaces(2.5)],
        }),
        time_signature: Some(TimeSignature {
            numerator: 4,
            denominator: 4,
        }),
        measures: vec![
            Measure {
                elements: vec![measure::Element::Note(Note {
                    y: StaffSpaces(1.5),
                    accidental: None,
                    duration: Duration {
                        value: duration::Value::Whole,
                        dots: None,
                    },
                })],
                ..Default::default()
            },
            Measure {
                elements: [
                    duration::Value::Whole,
                    duration::Value::Half,
                    duration::Value::Quarter,
                    duration::Value::Eighth,
                    duration::Value::Sixteenth,
                    duration::Value::ThirtySecond,
                    duration::Value::SixtyFourth,
                ]
                .into_iter()
                .enumerate()
                .map(|(index, value)| {
                    measure::Element::Note(Note {
                        y: ((index as f64) * 0.5).into(),
                        accidental: None,
                        duration: Duration { value, dots: None },
                    })
                })
                .collect(),
                ..Default::default()
            },
            Measure {
                elements: [
                    duration::Value::Whole,
                    duration::Value::Half,
                    duration::Value::Quarter,
                    duration::Value::Eighth,
                    duration::Value::Sixteenth,
                    duration::Value::ThirtySecond,
                    duration::Value::SixtyFourth,
                ]
                .into_iter()
                .map(|value| {
                    measure::Element::Rest(Rest {
                        duration: Duration { value, dots: None },
                    })
                })
                .collect(),
                bar_line: Barline::Final,
            },
        ],
    };

    let elements = staff.render(&metadata);

    let options = Options {
        symbol_font_name: metadata.font_name.clone(),
        text_font_family: metadata.engraving_defaults.text_font_family,
        staff_space_to_pixel_ratio: 15.0,
    };

    let svg = elements_to_svg_document(elements, &options).to_string();

    let mut output_path = root_path.to_path_buf();
    output_path.push("test.svg");

    println!("Writing to {}", output_path.display());

    let mut file = File::create(&output_path).unwrap();
    file.write_all(svg.as_bytes()).unwrap();
}
