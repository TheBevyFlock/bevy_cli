use std::process::Command;

use super::arg_builder::ArgBuilder;

pub(crate) const PACKAGE: &str = "wasm-bindgen-cli";
pub(crate) const PROGRAM: &str = "wasm-bindgen";

fn command() -> Command {
    Command::new(PROGRAM)
}

/// Bundle the WASM build for the web.
pub(crate) fn bundle(package_name: &str, is_release: bool) -> anyhow::Result<()> {
    let target = if is_release { "release" } else { "debug" };

    let status = command()
        .args(
            ArgBuilder::new()
                .add("--no-typescript")
                .add_with_value("--out-name", "bevy_game")
                .add_with_value("--out-dir", "web")
                .add_with_value("--target", "web")
                .add(format!(
                    "target/wasm32-unknown-unknown/{target}/{package_name}.wasm"
                )),
        )
        .status()?;

    if !status.success() {
        Err(anyhow::anyhow!("Failed to bundle project for the web."))
    } else {
        Ok(())
    }
}
