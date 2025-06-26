use std::{fs, time::Instant};

use tracing::info;

use crate::{
    bin_target::BinTarget,
    external_cli::{
        CommandExt, Package, arg_builder::ArgBuilder, cargo::install::AutoInstall,
        external_cli_args::ExternalCliArgs,
    },
};

pub(crate) const PACKAGE: &str = "wasm-opt";
pub(crate) const PROGRAM: &str = "wasm-opt";

/// Optimize the Wasm binary at the given path with wasm-opt.
pub(crate) fn optimize_path(
    bin_target: &BinTarget,
    auto_install: AutoInstall,
    external_args: &ExternalCliArgs,
) -> anyhow::Result<()> {
    let path = bin_target
        .artifact_directory
        .clone()
        .join(format!("{}_bg.wasm", bin_target.bin_name));

    let wasm_opt_args = ArgBuilder::new()
        .add_with_value("--output", &path)
        .arg(&path);

    let wasm_opt_args = match external_args {
        ExternalCliArgs::Enabled(enabled) => {
            if *enabled {
                // Use default args
                wasm_opt_args.args(["--strip-debug", "-Os"])
            } else {
                // Skip optimization if not enabled
                return Ok(());
            }
        }
        // Add the custom args provided by the user
        ExternalCliArgs::Args(args) => wasm_opt_args.args(args.clone()),
    };
    info!("optimizing with wasm-opt...");

    let start = Instant::now();
    let size_before = fs::metadata(&path)?.len();

    let package = Package {
        name: PACKAGE.into(),
        ..Default::default()
    };

    CommandExt::new(PROGRAM)
        .require_package(package)
        .args(wasm_opt_args)
        .ensure_status(auto_install)?;

    let size_after = fs::metadata(path)?.len();
    let size_reduction = 1. - (size_after as f32) / (size_before as f32);
    let duration = start.elapsed();

    info!(
        "finished in {duration:.2?}. Size reduced by {:.0}%.",
        size_reduction * 100.
    );

    Ok(())
}
