use crate::external_cli::cargo;
#[cfg(feature = "web")]
use crate::web::run::run_web;

pub use self::args::RunArgs;

pub mod args;

pub fn run(args: &mut RunArgs) -> anyhow::Result<()> {
    let metadata = cargo::metadata::metadata_with_args(["--no-deps"])?;

    #[cfg(feature = "web")]
    if args.is_web() {
        return run_web(args, &metadata);
    }

    let cargo_args = args.cargo_args_builder();
    // For native builds, wrap `cargo run`
    cargo::run::command().args(cargo_args).ensure_status()?;

    Ok(())
}
