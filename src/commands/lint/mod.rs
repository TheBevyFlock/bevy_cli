pub use args::*;
use tracing::error;

#[cfg(feature = "rustup")]
use crate::commands::lint::install::install_linter;
use crate::{
    bin_target::select_run_binary,
    commands::lint::install::list,
    config::CliConfig,
    external_cli::cargo::{
        self,
        install::{AutoInstall, is_installed},
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

    if let Some(LintSubcommands::List) = args.subcommand {
        return list();
    }

    #[cfg(feature = "rustup")]
    if let Some(LintSubcommands::Install(install_args)) = &args.subcommand {
        return install_linter(install_args, args.auto_install());
    }

    if is_installed(PROGRAM).is_none() {
        error!(
            "{} is not present, install {} via `bevy install lint`",
            PROGRAM, PROGRAM
        );
        return Ok(());
    }

    let metadata = cargo::metadata::metadata()?;

    let mut bin_target = select_run_binary(
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
    // Update the artifact directory based on the config, e.g. in case the `target` changed
    bin_target.update_artifact_directory(
        &metadata.target_directory,
        args.target().as_deref(),
        args.profile(),
        args.cargo_args.target_args.example.is_some(),
    );

    #[cfg(feature = "web")]
    if matches!(args.subcommand, Some(LintSubcommands::Web)) {
        use crate::web::profiles::configure_default_web_profiles;

        let mut profile_args = configure_default_web_profiles(&metadata)?;
        // `--config` args are resolved from left to right,
        // so the default configuration needs to come before the user args
        profile_args.append(&mut args.cargo_args.common_args.config);
        args.cargo_args.common_args.config = profile_args;
    }

    let cargo_args = args.cargo_args_builder();

    let mut cmd = crate::external_cli::CommandExt::new("bevy_lint");

    cmd.args(cargo_args)
        .env("RUSTFLAGS", args.cargo_args.common_args.rustflags.clone());

    let status = crate::external_cli::CommandExt::new("bevy_lint")
        // We do not want to automatically install a `bevy_lint` version.
        // The reason is that to pass the `Package`, we would need to look up the latest release on
        // GitHub since there is no easy way of specify "latest".
        .ensure_status(AutoInstall::Never)?;

    ensure!(
        status.success(),
        "`bevy_lint` exited with a non-zero exit code."
    );

    Ok(())
}
