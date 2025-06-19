use std::{fs, time::Instant};

use tracing::info;

use crate::{
    bin_target::BinTarget,
    external_cli::{
        CommandExt, Package, cargo::install::AutoInstall, external_cli_args::ExternalCliArgs,
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
    let args = match external_args {
        ExternalCliArgs::Enabled(enabled) => {
            if *enabled {
                // Use default args
                vec![
                    "--strip-debug".to_string(),
                    "-Os".to_string(),
                    "-o".to_string(),
                ]
            } else {
                // Skip optimization if not enabled
                return Ok(());
            }
        }
        ExternalCliArgs::Args(args) => args.clone(),
    };

    let path = bin_target
        .artifact_directory
        .clone()
        .join(format!("{}_bg.wasm", bin_target.bin_name));
    info!("optimizing with wasm-opt...");

    let start = Instant::now();
    let size_before = fs::metadata(&path)?.len();

    let package = Package {
        name: PACKAGE.into(),
        ..Default::default()
    };

    CommandExt::new(PROGRAM)
        .require_package(package)
        .args(args)
        .arg(&path)
        .arg(&path)
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
