use semver::{Comparator, Version, VersionReq};

use crate::bin_target::BinTarget;

use super::{CommandExt, arg_builder::ArgBuilder, cargo::metadata::Metadata};

pub(crate) const PACKAGE: &str = "wasm-bindgen-cli";
pub(crate) const PROGRAM: &str = "wasm-bindgen";

/// Bundle the Wasm build for the web.
pub(crate) fn bundle(metadata: &Metadata, bin_target: &BinTarget) -> anyhow::Result<()> {
    let original_wasm = bin_target
        .artifact_directory
        .clone()
        .join(format!("{}.wasm", bin_target.bin_name));

    let Version {
        major,
        minor,
        patch,
        pre,
        build: _,
    } = metadata
        .packages
        .iter()
        .find(|package| package.name == "wasm-bindgen")
        .map(|package| package.version.clone())
        .ok_or_else(|| anyhow::anyhow!("Failed to find wasm-bindgen"))?;

    CommandExt::new(PROGRAM)
        .require_package(
            PACKAGE,
            VersionReq {
                comparators: vec![Comparator {
                    // The wasm-bindgen versions need to match exactly
                    op: semver::Op::Exact,
                    major,
                    minor: Some(minor),
                    patch: Some(patch),
                    pre,
                }],
            },
        )
        .args(
            ArgBuilder::new()
                .arg("--no-typescript")
                .add_with_value("--out-name", &bin_target.bin_name)
                .add_with_value("--out-dir", bin_target.artifact_directory.to_string_lossy())
                .add_with_value("--target", "web")
                .arg(original_wasm.to_string_lossy()),
        )
        .ensure_status()?;

    Ok(())
}
