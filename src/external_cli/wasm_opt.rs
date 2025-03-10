use std::{fs, time::Instant};

use tracing::info;

use crate::{external_cli::CommandExt, web::bin_target::BinTarget};

pub(crate) const PACKAGE: &str = "wasm-opt";
pub(crate) const PROGRAM: &str = "wasm-opt";

/// Optimize the Wasm binary at the given path with wasm-opt.
pub(crate) fn optimize_path(bin_target: &BinTarget) -> anyhow::Result<()> {
    let path = bin_target
        .artifact_directory
        .clone()
        .join(format!("{}_bg.wasm", bin_target.bin_name));
    info!("Optimizing with wasm-opt...");

    let start = Instant::now();
    let size_before = fs::metadata(&path)?.len();

    CommandExt::new(PROGRAM)
        .arg("--strip-debug")
        .arg("-Oz")
        .arg("-o")
        .arg(&path)
        .arg(&path)
        .ensure_status()?;

    let size_after = fs::metadata(path)?.len();
    let size_reduction = 1. - (size_after as f32) / (size_before as f32);
    let duration = start.elapsed();

    info!(
        "Finished in {duration:.2?}. Size reduced by {:.0}%.",
        size_reduction * 100.
    );

    Ok(())
}
