#!/usr/bin/env bash

cargo run --bin smufl-gen -- submodules/smufl/metadata/glyphnames.json > smufl/src/glyph-temp.rs
mv smufl/src/glyph-temp.rs smufl/src/glyph.rs
cargo fmt
