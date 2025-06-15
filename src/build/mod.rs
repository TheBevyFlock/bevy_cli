//! Provides functionalities to build a Bevy app targeting either native or web platforms.

use args::BuildArgs;

#[cfg(feature = "web")]
use crate::web::build::build_web;
use crate::{bin_target::select_run_binary, config::CliConfig, external_cli::cargo};

pub mod args;

/// Tries to build the project with the given [`BuildArgs`].
///
/// # Errors
///
/// will error if the build process can not be completed
pub fn build(args: &mut BuildArgs) -> anyhow::Result<()> {
    let metadata = cargo::metadata::metadata()?;

    let bin_target = select_run_binary(
        &metadata,
        args.cargo_args.package_args.package.as_deref(),
        args.cargo_args.target_args.bin.as_deref(),
        args.cargo_args.target_args.example.as_deref(),
        args.target().as_deref(),
        args.profile(),
    )?;

    let config = CliConfig::for_package(
        &metadata,
        bin_target.package,
        args.is_web(),
        args.is_release(),
    )?;

    args.apply_config(&config);
    // Re-run the bin target selection, as the config can affect e.g. the path to the artifact
    let bin_target = select_run_binary(
        &metadata,
        args.cargo_args.package_args.package.as_deref(),
        args.cargo_args.target_args.bin.as_deref(),
        args.cargo_args.target_args.example.as_deref(),
        args.target().as_deref(),
        args.profile(),
    )?;

    #[cfg(feature = "web")]
    if args.is_web() {
        build_web(args, &metadata, &bin_target)?;
        return Ok(());
    }

    let cargo_args = args.cargo_args_builder();
    cargo::build::command()
        .args(cargo_args)
        .ensure_status(args.auto_install())?;

    Ok(())
}
