use std::process::Command;

use super::arg_builder::ArgBuilder;

pub(crate) const PACKAGE: &str = "wasm-bindgen-cli";
pub(crate) const PROGRAM: &str = "wasm-bindgen";

fn command() -> Command {
    Command::new(PROGRAM)
}

/// Determine the path to the folder where the WASM build artifacts are stored.
pub(crate) fn get_target_folder(is_release: bool) -> String {
    let target = if is_release { "release" } else { "debug" };
    format!("target/wasm32-unknown-unknown/{target}")
}

/// Bundle the WASM build for the web.
pub(crate) fn bundle(package_name: &str, is_release: bool) -> anyhow::Result<()> {
    let target_folder = get_target_folder(is_release);

    let status = command()
        .args(
            ArgBuilder::new()
                .add("--no-typescript")
                .add_with_value("--out-name", "bevy_app")
                .add_with_value("--out-dir", &target_folder)
                .add_with_value("--target", "web")
                .add(format!("{target_folder}/{package_name}.wasm")),
        )
        .status()?;

    if !status.success() {
        Err(anyhow::anyhow!("Failed to bundle project for the web."))
    } else {
        Ok(())
    }
}
