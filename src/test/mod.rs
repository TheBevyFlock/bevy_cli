//! Provides functionalities to test a Bevy app targeting either native or web platforms.

use args::TestArgs;

//#[cfg(feature = "web")]
// use crate::web::build::test_web;
use crate::{
    bin_target::select_run_binary, config::CliConfig, external_cli::cargo,
    web::profiles::configure_default_web_profiles,
};

pub mod args;

/// Tries to test the project with the given [`TestArgs`].
///
/// # Errors
///
/// will error if the test process can not be completed
pub fn test(args: &mut TestArgs) -> anyhow::Result<()> {
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

    #[cfg(feature = "web")]
    if args.is_web() {
        let mut profile_args = configure_default_web_profiles(&metadata)?;
        // `--config` args are resolved from left to right,
        // so the default configuration needs to come before the user args
        profile_args.append(&mut args.cargo_args.common_args.config);
        args.cargo_args.common_args.config = profile_args;

        let cargo_args = args.cargo_args_builder();
        cargo::test::command()
            .args(cargo_args)
            .env("RUSTFLAGS", args.cargo_args.common_args.rustflags.clone())
            .ensure_status(TestArgs::auto_install())?;
        return Ok(());
    }

    let cargo_args = args.cargo_args_builder();
    cargo::test::command()
        .args(cargo_args)
        .env("RUSTFLAGS", args.cargo_args.common_args.rustflags.clone())
        .ensure_status(TestArgs::auto_install())?;

    Ok(())
}
