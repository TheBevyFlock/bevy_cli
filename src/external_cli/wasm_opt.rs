use std::{fs, time::Instant};

use semver::VersionReq;
use tracing::info;

use crate::{
    bin_target::BinTarget,
    external_cli::{CommandExt, cargo::install::AutoInstall},
};

pub(crate) const PACKAGE: &str = "wasm-opt";
pub(crate) const PROGRAM: &str = "wasm-opt";

/// Optimize the Wasm binary at the given path with wasm-opt.
pub(crate) fn optimize_path(
    bin_target: &BinTarget,
    auto_install: AutoInstall,
) -> anyhow::Result<()> {
    let path = bin_target
        .artifact_directory
        .clone()
        .join(format!("{}_bg.wasm", bin_target.bin_name));
    info!("Optimizing with wasm-opt...");

    let start = Instant::now();
    let size_before = fs::metadata(&path)?.len();

    CommandExt::new(PROGRAM)
        .require_package(PACKAGE, VersionReq::STAR)
        .arg("--strip-debug")
        .arg("-Os")
        .arg("-o")
        .arg(&path)
        .arg(&path)
        .ensure_status(auto_install)?;

    let size_after = fs::metadata(path)?.len();
    let size_reduction = 1. - (size_after as f32) / (size_before as f32);
    let duration = start.elapsed();

    info!(
        "Finished in {duration:.2?}. Size reduced by {:.0}%.",
        size_reduction * 100.
    );

    Ok(())
}
