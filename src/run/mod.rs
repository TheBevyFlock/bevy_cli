#[cfg(feature = "web")]
use crate::web::run::run_web;
use crate::{bin_target::select_run_binary, external_cli::cargo};

pub use self::args::RunArgs;

pub mod args;

pub fn run(args: &mut RunArgs) -> anyhow::Result<()> {
    let metadata = cargo::metadata::metadata_with_args(["--no-deps"])?;

    let bin_target = select_run_binary(
        &metadata,
        args.cargo_args.package_args.package.as_deref(),
        args.cargo_args.target_args.bin.as_deref(),
        args.cargo_args.target_args.example.as_deref(),
        args.cargo_args
            .compilation_args
            .target(args.is_web())
            .as_deref(),
        args.cargo_args.compilation_args.profile(args.is_web()),
    )?;

    #[cfg(feature = "web")]
    if args.is_web() {
        return run_web(args, &metadata, &bin_target);
    }

    let cargo_args = args.cargo_args_builder();
    // For native builds, wrap `cargo run`
    cargo::run::command().args(cargo_args).ensure_status()?;

    Ok(())
}
