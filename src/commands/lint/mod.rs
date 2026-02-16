pub use args::*;
use tracing::{error, info};

#[cfg(feature = "rustup")]
use crate::commands::lint::install::install_linter;
use crate::{
    commands::{get_package, lint::install::list},
    config::CliConfig,
    external_cli::{
        CommandExt,
        cargo::{
            self,
            install::{AutoInstall, is_installed},
        },
    },
};

mod args;
mod install;

/// Runs `bevy_lint`, if it is installed, with the given arguments.
///
/// Calling `lint(vec!["--workspace"])` is equivalent to calling `bevy_lint --workspace` in the
/// terminal.
pub fn lint(args: &mut LintArgs) -> anyhow::Result<()> {
    const PROGRAM: &str = "bevy_lint";
    use anyhow::ensure;

    if let Some(LintSubcommands::List) = args.subcommand
        && !args.version
        && !args.fix
    {
        return list();
    }

    #[cfg(feature = "rustup")]
    if let Some(LintSubcommands::Install(install_args)) = &args.subcommand
        && !args.version
        && !args.fix
    {
        return install_linter(install_args);
    }

    if is_installed(PROGRAM).is_none() {
        error!(
            "{} is not present, install {} via `bevy lint install`",
            PROGRAM, PROGRAM
        );
        return Ok(());
    }

    let status = build_lint_cmd(args)?
        // We do not want to automatically install a `bevy_lint` version.
        // The reason is that to pass the `Package`, we would need to look up the latest release on
        // GitHub since there is no easy way of specify "latest".
        .ensure_status(AutoInstall::Never)
        .inspect_err(|_| {
            #[cfg(feature = "web")]
            use crate::web::getrandom::getrandom_web_feature_config;

            // If the build failed, check if the user has configured `getrandom` correctly
            #[cfg(feature = "web")]
            if let Some(target) = args.target()
                && let Ok(Some(feature_config)) = getrandom_web_feature_config(&target)
            {
                tracing::warn!(
                    "You have to enable the `getrandom` web feature in your Cargo.toml:\n\n{feature_config}"
                );
            }
        })?;

    ensure!(
        status.success(),
        "`bevy_lint` exited with a non-zero exit code."
    );

    Ok(())
}

fn build_lint_cmd(args: &mut LintArgs) -> anyhow::Result<CommandExt> {
    let mut cmd = crate::external_cli::CommandExt::new("bevy_lint");

    // only append `--version`
    if args.version {
        cmd.arg("--version");
        return Ok(cmd);
    }

    // All additional first party `bevy_lint` arguments need to be the first arguments so
    // the `forward_args` apply to them.
    if args.fix {
        cmd.arg("--fix");
    }

    // Append all forward args. These should come before the
    // cargo check args since the forward args would target `bevy_lint` and
    // `bevy_lint` appends all additional arguments that are not recognized
    // to `cargo check`.
    // The forward args are used to support `bevy_lint` arguments that do not yet have first party
    // support in the cli.
    if !args.forward_args.is_empty() {
        cmd.args(args.forward_args.iter());
    }

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

    #[cfg(feature = "web")]
    if matches!(args.subcommand, Some(LintSubcommands::Web)) {
        use tracing::info;

        use crate::web::{
            getrandom::apply_getrandom_backend, profiles::configure_default_web_profiles,
        };

        let mut profile_args = configure_default_web_profiles(&metadata)?;
        // `--config` args are resolved from left to right,
        // so the default configuration needs to come before the user args
        profile_args.append(&mut args.cargo_args.common_args.config);
        args.cargo_args.common_args.config = profile_args;

        if apply_getrandom_backend(&metadata, &mut args.cargo_args.common_args) {
            info!("automatically configuring `getrandom` web backend");
        }
    }

    // If a specific example was passed, extend the already present features with the
    // required_features from this example.
    if let Some(example) = &args.cargo_args.target_args.example
    // Search in the current workspace packages for an `example` target that matches the given
    // example name.
        && let Some(example_target) = metadata
            .workspace_packages()
            .iter()
            .flat_map(|p| p.targets.clone())
            .find(|t| t.name.as_str() == example && t.kind.contains(&cargo_metadata::TargetKind::Example))
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

    let cargo_args = args.cargo_args_builder();

    cmd.args(cargo_args)
        .env("RUSTFLAGS", args.cargo_args.common_args.rustflags.clone());

    Ok(cmd)
}
