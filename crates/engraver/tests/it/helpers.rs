use std::{
    env,
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use color_eyre::{eyre::Context, Result};
use engraver::{render::input::Staff, svg};
use once_cell::sync::OnceCell;
use smufl::Metadata;

macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of(f)
            .strip_prefix("it::")
            .unwrap()
            .strip_suffix("::f")
            .unwrap()
    }};
}

#[macro_export]
macro_rules! assert_staff_snapshot {
    ($staff:ident) => {{
        if let $crate::helpers::SnapshotResult::Different { expected, actual } =
            $crate::helpers::_assert_staff_snapshot($staff, function_name!()).unwrap()
        {
            panic!("snapshots do not match\n\nExpected: {expected}\nActual: {actual}",);
        }
    }};
    ($input:literal) => {{
        let staff = engraver_parser::parse_staff($input).unwrap().into();

        if let $crate::helpers::SnapshotResult::Different { expected, actual } =
            $crate::helpers::_assert_staff_snapshot(staff, function_name!()).unwrap()
        {
            panic!(
                "snapshots do not match\n\nInput: '{}'\nExpected: {}\nActual: {}",
                $input, expected, actual
            );
        }
    }};
}

fn metadata() -> &'static Metadata {
    static INSTANCE: OnceCell<Metadata> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut font_metadata_path = root_path();
        font_metadata_path.push("../../submodules/bravura/redist/bravura_metadata.json");

        let file = File::open(font_metadata_path).unwrap();
        let reader = BufReader::new(file);

        smufl::Metadata::from_reader(reader).unwrap()
    })
}

fn should_overwrite_snapshots() -> bool {
    static INSTANCE: OnceCell<bool> = OnceCell::new();
    *INSTANCE.get_or_init(|| {
        std::env::var("ENGRAVER_OVERWRITE_SNAPSHOTS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or_default()
    })
}

fn root_path() -> PathBuf {
    let cargo_manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set");
    Path::new(&cargo_manifest_dir).to_path_buf()
}

fn escaped_snapshot_name(name: &str) -> String {
    name.replace(':', "_")
}

fn snapshot_path(name: &str) -> Result<PathBuf> {
    let mut snapshot_path = root_path();
    snapshot_path.push("tests");
    snapshot_path.push("it");
    snapshot_path.push("snapshots");

    fs::create_dir_all(&snapshot_path)?;

    snapshot_path.push(escaped_snapshot_name(name));
    snapshot_path.set_extension("svg");

    Ok(snapshot_path.to_path_buf())
}

fn new_snapshot_path(name: &str) -> Result<PathBuf> {
    let mut snapshot_path = env::temp_dir();
    snapshot_path.push("engraver");
    snapshot_path.push("it");
    snapshot_path.push("snapshots");

    fs::create_dir_all(&snapshot_path)?;

    snapshot_path.push(escaped_snapshot_name(name));
    snapshot_path.set_extension("svg");

    Ok(snapshot_path.to_path_buf())
}

pub enum SnapshotResult {
    New,
    Same,
    Different { expected: String, actual: String },
}

pub fn _assert_staff_snapshot(staff: Staff, name: &str) -> Result<SnapshotResult> {
    let metadata = metadata();
    let elements = staff
        .render(metadata)
        .wrap_err(format!("Failed to render '{}' snapshot", name))?;

    let options = svg::Options {
        symbol_font_name: metadata.font_name.clone(),
        text_font_family: metadata.engraving_defaults.text_font_family.clone(),
        staff_space_to_pixel_ratio: 15.0,
    };

    let contents = svg::elements_to_svg_document(elements, &options).to_string();

    let path = snapshot_path(name)?;

    match (should_overwrite_snapshots(), fs::read_to_string(&path)) {
        (false, Ok(existing)) => {
            if contents != existing {
                let temp_path = new_snapshot_path(name)?;
                fs::write(&temp_path, &contents)?;

                return Ok(SnapshotResult::Different {
                    expected: path.display().to_string(),
                    actual: temp_path.display().to_string(),
                });
            }

            Ok(SnapshotResult::Same)
        }
        _ => {
            fs::write(&path, contents).wrap_err(format!(
                "Could not write '{}' snapshot to path: {}",
                name,
                path.display()
            ))?;

            Ok(SnapshotResult::New)
        }
    }
}
