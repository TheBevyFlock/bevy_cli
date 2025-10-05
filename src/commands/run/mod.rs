//! Provides functionalities to run a Bevy app targeting either native or web platforms.

pub use self::args::*;
#[cfg(feature = "web")]
use crate::web::run::run_web;
use crate::{commands::get_default_package, config::CliConfig, external_cli::cargo};

mod args;

/// Tries to run the project with the given [`RunArgs`].
///
/// # Errors
///
/// will error if the build process can not be completed
pub fn run(args: &mut RunArgs) -> anyhow::Result<()> {
    let metadata = cargo::metadata::metadata()?;

    let package = get_default_package(
        &metadata,
        args.cargo_args.package_args.package.as_ref(),
        true,
    )?;

    // apply the package specific config, otherwise use the default config (this happens when
    // `bevy build` was called from a workspace root with no package selection (we do not support
    // workspace config at the moment).
    let mut config = if let Some(package) = package {
        CliConfig::for_package(&metadata, package, args.is_web(), args.is_release())?
    } else {
        CliConfig::default()
    };

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
