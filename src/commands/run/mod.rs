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

    // If the `--package` arg was passed, search for the given package in the workspace, otherwise
    // get the root package.
    let package = if let Some(package_name) = &args.cargo_args.package_args.package {
        let workspace_packages = metadata.workspace_packages();
        workspace_packages
            .iter()
            .find(|package| package.name.as_str() == package_name)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Failed to find package {package_name}"))?
    } else {
        metadata
            .root_package()
            .ok_or_else(|| anyhow::anyhow!("Failed to determain root package to build"))?
    };

    let mut config = CliConfig::for_package(&metadata, package, args.is_web(), args.is_release())?;

    // Read config files hierarchically from the current directory, merge them,
    // apply environment variables, and resolve relative paths.
    let cargo_config = cargo_config2::Config::load()?;
    config.append_cargo_config_rustflags(args.target(), &cargo_config)?;

    args.apply_config(&config);

    #[cfg(feature = "web")]
    if args.is_web() {
        return run_web(args, &metadata);
    }

    let cargo_args = args.cargo_args_builder();

    // For native builds, wrap `cargo run`
    cargo::run::command()
        .args(cargo_args)
        .env("RUSTFLAGS", args.cargo_args.common_args.rustflags.clone())
        .ensure_status(args.auto_install())?;

    Ok(())
}
