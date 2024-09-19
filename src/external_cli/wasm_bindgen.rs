#![expect(dead_code, reason = "Will be used for the build/run commands")]

use std::process::Command;

use super::arg_builder::ArgBuilder;

pub(crate) const PACKAGE: &str = "wasm-bindgen-cli";
pub(crate) const PROGRAM: &str = "wasm-bindgen";

/// Determine the path to the folder where the Wasm build artifacts are stored.
pub(crate) fn get_target_folder(profile: &str) -> String {
    format!("target/wasm32-unknown-unknown/{profile}")
}

/// Bundle the Wasm build for the web.
pub(crate) fn bundle(package_name: &str, profile: &str) -> anyhow::Result<()> {
    let target_folder = get_target_folder(profile);

    let status = Command::new(PROGRAM)
        .args(
            ArgBuilder::new()
                .arg("--no-typescript")
                .add_with_value("--out-name", "bevy_app")
                .add_with_value("--out-dir", &target_folder)
                .add_with_value("--target", "web")
                .arg(format!("{target_folder}/{package_name}.wasm")),
        )
        .status()?;

    anyhow::ensure!(status.success(), "Failed to bundle project for the web.");
    Ok(())
}
