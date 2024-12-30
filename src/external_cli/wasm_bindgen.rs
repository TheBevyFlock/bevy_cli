use semver::Version;
use std::{process::Command, str::FromStr};

use crate::{external_cli::CommandHelpers, run::BinTarget};

use super::arg_builder::ArgBuilder;

pub(crate) const PACKAGE: &str = "wasm-bindgen-cli";
pub(crate) const PROGRAM: &str = "wasm-bindgen";

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

/// Transforms the output from `wasm-bindgen --version` into a [Version].
///
/// # Errors
///
/// * The `stdout` is not valid UTF-8.
/// * The output does not match the expected format: `wasm-bindgen <version>`.
/// * The version string cannot be parsed into a `semver::Version`.
///
/// # Examples
/// ```
/// let stdout = b"wasm-bindgen 0.2.99".to_vec();
/// let version = wasm_bindgen_cli_version(stdout).unwrap();
/// assert_eq!(version, Version::new(0, 2, 99));
/// ```
pub(crate) fn wasm_bindgen_cli_version(stdout: Vec<u8>) -> anyhow::Result<Version> {
    let stdout = String::from_utf8(stdout)?;
    // Example stdout from `wasm-bindgen --version`: wasm-bindgen 0.2.99
    stdout
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "unexpected output format: {}, expected format to be: `wasm-bindgen <version>`",
                stdout
            )
        })
        .and_then(|version| Version::from_str(version).map_err(|e| anyhow::anyhow!(e)))
}
