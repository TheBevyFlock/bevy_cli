use std::{fs, path::Path, time::Instant};

use anyhow::Context as _;

use crate::run::BinTarget;

/// Optimize the binary with wasm-opt.
pub(crate) fn optimize_bin(bin_target: &BinTarget) -> anyhow::Result<()> {
    let wasm_path = bin_target
        .artifact_directory
        .clone()
        .join(format!("{}_bg.wasm", bin_target.bin_name));

    optimize_path(&wasm_path)
}

/// Optimize the Wasm binary at the given path with wasm-opt.
fn optimize_path(path: &Path) -> anyhow::Result<()> {
    println!("Optimizing with wasm-opt...");

    let start = Instant::now();
    let size_before = fs::metadata(path)?.len();

    wasm_opt::OptimizationOptions::new_optimize_for_size()
        .run(path, path)
        .context("failed to optimize with wasm-opt")?;

    let size_after = fs::metadata(path)?.len();
    let size_reduction = 1. - (size_after as f32) / (size_before as f32);
    let duration = start.elapsed();

    println!(
        "    Finished in {duration:.2?}. Size reduced by {:.0}%.",
        size_reduction * 100.
    );

    Ok(())
}
