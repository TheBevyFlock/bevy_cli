#[cfg(feature = "web")]
use crate::web::run::run_web;
use crate::{bin_target::select_run_binary, config::CliConfig, external_cli::cargo};

pub use self::args::RunArgs;

pub mod args;

pub fn run(args: &mut RunArgs) -> anyhow::Result<()> {
    let metadata = cargo::metadata::metadata_with_args(["--no-deps"])?;

    let bin_target = select_run_binary(
        &metadata,
        args.cargo_args.package_args.package.as_deref(),
        args.cargo_args.target_args.bin.as_deref(),
        args.cargo_args.target_args.example.as_deref(),
        args.target().as_deref(),
        args.profile(),
    )?;

    let config = CliConfig::for_package(&metadata, bin_target.package, true, args.is_release())?;
    args.apply_config(&config);

    #[cfg(feature = "web")]
    if args.is_web() {
        return run_web(args, config.rustflags().as_deref(), &metadata, &bin_target);
    }

    let cargo_args = args.cargo_args_builder();
    // For native builds, wrap `cargo run`
    cargo::run::command()
        .args(cargo_args)
        .env("RUSTFLAGS", config.rustflags())
        .ensure_status()?;

    Ok(())
}
