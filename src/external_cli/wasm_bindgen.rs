use std::{path::Path, process::Command};

use crate::{external_cli::CommandHelpers, run::BinTarget};

use super::arg_builder::ArgBuilder;

pub(crate) const PACKAGE: &str = "wasm-bindgen-cli";
pub(crate) const PROGRAM: &str = "wasm-bindgen";

/// Determine the path to the folder where the Wasm build artifacts are stored.
pub(crate) fn get_target_folder(profile: &str) -> String {
    format!("target/wasm32-unknown-unknown/{profile}")
}

/// Bundle the Wasm build for the web.
pub(crate) fn bundle(bin_target: &BinTarget) -> anyhow::Result<()> {
    let original_wasm = bin_target
        .artifact_directory
        .clone()
        .join(format!("{}.wasm", bin_target.bin_name));

    Command::new(PROGRAM)
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
