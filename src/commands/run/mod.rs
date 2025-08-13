//! Provides functionalities to run a Bevy app targeting either native or web platforms.

pub use self::args::*;
#[cfg(feature = "web")]
use crate::web::run::run_web;
use crate::{bin_target::select_run_binary, config::CliConfig, external_cli::cargo};

mod args;

/// Tries to run the project with the given [`RunArgs`].
///
/// # Errors
///
/// will error if the build process can not be completed
pub fn run(args: &mut RunArgs) -> anyhow::Result<()> {
    let metadata = cargo::metadata::metadata()?;

    let mut bin_target = select_run_binary(
        &metadata,
        args.cargo_args.package_args.package.as_deref(),
        args.cargo_args.target_args.bin.as_deref(),
        args.cargo_args.target_args.example.as_deref(),
        args.target().as_deref(),
        args.profile(),
    )?;

    let mut config = CliConfig::for_package(
        &metadata,
        bin_target.package,
        args.is_web(),
        args.is_release(),
    )?;

    // Read config files hierarchically from the current directory, merge them,
    // apply environment variables, and resolve relative paths.
    let cargo_config = cargo_config2::Config::load()?;
    config.append_cargo_config_rustflags(args.target(), &cargo_config)?;

    args.apply_config(&config);
    // Update the artifact directory based on the config, e.g. in case the `target` changed
    bin_target.update_artifact_directory(
        &metadata.target_directory,
        args.target().as_deref(),
        args.profile(),
        args.cargo_args.target_args.example.is_some(),
    );

    #[cfg(feature = "web")]
    if args.is_web() {
        return run_web(args, &metadata, &bin_target);
    }

    let cargo_args = args.cargo_args_builder();

    // For native builds, wrap `cargo run`
    cargo::run::command()
        .args(cargo_args)
        .env("RUSTFLAGS", args.cargo_args.common_args.rustflags.clone())
        .ensure_status(args.auto_install())?;

    Ok(())
}
