//! Provides functionalities to test a Bevy app targeting either native or web platforms.
#[cfg(feature = "web")]
use crate::web::profiles::configure_default_web_profiles;
use crate::{config::CliConfig, external_cli::cargo};
use args::TestArgs;

pub mod args;

/// Tries to test the project with the given [`TestArgs`].
///
/// # Errors
///
/// will error if the test process can not be completed
pub fn test(args: &mut TestArgs) -> anyhow::Result<()> {
    let metadata = cargo::metadata::metadata()?;

    // The package specified with the `-p` argument or if none is present take the
    // current root_package
    let package = if let Some(package) = &args.cargo_args.package_args.package {
        metadata
            .packages
            .iter()
            .find(|p| p.name.as_str() == package)
    } else {
        metadata.root_package()
    };

    let mut config = CliConfig::default();
    if let Some(package) = package {
        config = CliConfig::for_package(&metadata, package, args.is_web(), args.is_release())?;
    }

    args.apply_config(&config);

    #[cfg(feature = "web")]
    if args.is_web() {
        let mut profile_args = configure_default_web_profiles(&metadata)?;
        // `--config` args are resolved from left to right,
        // so the default configuration needs to come before the user args
        profile_args.append(&mut args.cargo_args.common_args.config);
        args.cargo_args.common_args.config = profile_args;
    }

    let cargo_args = args.cargo_args_builder();

    cargo::test::command(args.test_name.as_deref())
        .args(cargo_args)
        .env("RUSTFLAGS", args.cargo_args.common_args.rustflags.clone())
        .ensure_status(TestArgs::auto_install())?;

    Ok(())
}
