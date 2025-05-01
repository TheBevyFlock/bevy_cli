use args::RunWebArgs;

#[cfg(feature = "web")]
use crate::web::run::run_web;
use crate::{bin_target::select_run_binary, config::CliConfig, external_cli::cargo};

pub use self::args::RunArgs;

pub mod args;

pub fn run(args: &mut RunArgs) -> anyhow::Result<()> {
    let metadata = cargo::metadata::metadata()?;

    // Check if the profile, passed as an argument matches a default profile from the CLI.
    // if it matches, set the flags accordingly.
    if let Some(profile) = &args.cargo_args.compilation_args.profile {
        if profile == "release" {
            args.cargo_args.compilation_args.is_release = true;
        } else if profile == "web-release" {
            args.cargo_args.compilation_args.is_release = true;
            args.subcommand = Some(args::RunSubcommands::Web(RunWebArgs::default()));
        } else if profile == "web" {
            args.subcommand = Some(args::RunSubcommands::Web(RunWebArgs::default()));
        }
    }

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
