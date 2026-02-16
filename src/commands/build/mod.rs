//! Provides functionalities to build a Bevy app targeting either native or web platforms.

pub use args::*;
use cargo_metadata::TargetKind;
use tracing::info;

#[cfg(feature = "web")]
use crate::web::build::build_web;
use crate::{commands::get_package, config::CliConfig, external_cli::cargo};

mod args;

/// Tries to build the project with the given [`BuildArgs`].
///
/// # Errors
///
/// will error if the build process can not be completed
pub fn build(args: &mut BuildArgs) -> anyhow::Result<()> {
    let metadata = cargo::metadata::metadata()?;

    let package = get_package(
        &metadata,
        args.cargo_args.package_args.package.as_ref(),
        args.cargo_args.target_args.is_examples,
        false,
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

    // If a specific example was passed, extend the already present features with the
    // required_features from this example.
    if let Some(example) = &args.cargo_args.target_args.example
    // Search in the current workspace packages for an `example` target that matches the given
    // example name.
        && let Some(example_target) = metadata
            .workspace_packages()
            .iter()
            .flat_map(|p| p.targets.clone())
            .find(|t| t.name.as_str() == example && t.kind.contains(&TargetKind::Example))
    {
        let required_features = example_target.required_features;

        info!(
            "enabling required_features: {:?}, for example: {example}",
            required_features
        );

        args.cargo_args
            .feature_args
            .features
            .extend(required_features);
    }

    #[cfg(feature = "web")]
    if args.is_web() {
        build_web(args, &metadata)?;
        return Ok(());
    }

    let cargo_args = args.cargo_args_builder();
    cargo::build::command()
        .args(cargo_args)
        .env("RUSTFLAGS", args.rustflags())
        .ensure_status(args.auto_install())?;

    Ok(())
}
