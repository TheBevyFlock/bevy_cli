pub use args::*;
#[cfg(feature = "rustup")]
use serde::Deserialize;

#[cfg(feature = "rustup")]
use crate::external_cli::Package;

mod args;

/// Represents the contents of `rust-toolchain.toml`.
#[cfg(feature = "rustup")]
#[derive(Deserialize)]
struct RustToolchain {
    toolchain: Toolchain,
}

#[cfg(feature = "rustup")]
#[derive(Deserialize)]
struct Toolchain {
    channel: String,
}

/// Runs `bevy_lint`, if it is installed, with the given arguments.
///
/// Calling `lint(vec!["--workspace"])` is equivalent to calling `bevy_lint --workspace` in the
/// terminal.
pub fn lint(args: &mut LintArgs) -> anyhow::Result<()> {
    use anyhow::ensure;

    #[cfg(feature = "web")]
    use crate::commands::lint::args::LintSubcommands;
    use crate::{bin_target::select_run_binary, config::CliConfig, external_cli::cargo};

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

    #[cfg(feature = "rustup")]
    cmd.require_package(parse_required_package()?);

    cmd.args(cargo_args)
        .env("RUSTFLAGS", args.cargo_args.common_args.rustflags.clone());

    let status = cmd.ensure_status(args.auto_install())?;

    ensure!(
        status.success(),
        "`bevy_lint` exited with a non-zero exit code."
    );

    Ok(())
}

#[cfg(feature = "rustup")]
fn parse_required_package() -> anyhow::Result<Package> {
    use anyhow::Context;
    const RUST_TOOLCHAIN: &str = include_str!("../../../rust-toolchain.toml");
    const BEVY_LINT_TAG: &str = "lint-v0.3.0";
    const PACKAGE: &str = "bevy_lint";
    const GIT_URL: &str = "https://github.com/TheBevyFlock/bevy_cli.git";

    let rust_toolchain: RustToolchain =
        toml::from_str(RUST_TOOLCHAIN).context("Failed to parse `rust-toolchain.toml`.")?;

    Ok(Package {
        name: PACKAGE.into(),
        required_toolchain: Some(rust_toolchain.toolchain.channel),
        git: Some(GIT_URL.to_string()),
        tag: Some(BEVY_LINT_TAG.to_string()),
        ..Default::default()
    })
}
