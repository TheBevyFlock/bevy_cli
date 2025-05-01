use args::BuildArgs;

#[cfg(feature = "web")]
use crate::web::build::build_web;
use crate::{bin_target::select_run_binary, config::CliConfig, external_cli::cargo};

pub mod args;

pub fn build(args: &mut BuildArgs) -> anyhow::Result<()> {
    let mut is_web = args.is_web();
    let metadata = cargo::metadata::metadata()?;

    let bin_target = select_run_binary(
        &metadata,
        args.cargo_args.package_args.package.as_deref(),
        args.cargo_args.target_args.bin.as_deref(),
        args.cargo_args.target_args.example.as_deref(),
        args.target().as_deref(),
        args.profile(),
    )?;

    // Check if the profile, passed as an argument matches a default profile from the CLI.
    // if it matches, set the flags accordingly.
    if let Some(profile) = &args.cargo_args.compilation_args.profile {
        if profile == "release" {
            args.cargo_args.compilation_args.is_release = true;
        } else if profile == "web-release" {
            args.cargo_args.compilation_args.is_release = true;
            is_web = true;
        } else if profile == "web" {
            is_web = true;
        }
    }

    let config = CliConfig::for_package(&metadata, bin_target.package, is_web, args.is_release())?;

    args.apply_config(&config);

    #[cfg(feature = "web")]
    if is_web {
        build_web(args, &metadata, &bin_target)?;
        return Ok(());
    }

    let cargo_args = args.cargo_args_builder();
    cargo::build::command()
        .args(cargo_args)
        .ensure_status(args.auto_install())?;

    Ok(())
}
