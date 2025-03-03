use crate::external_cli::{cargo, CommandHelpers};
#[cfg(feature = "web")]
use crate::web::run::run_web;

pub use self::args::RunArgs;

pub mod args;

pub fn run(args: &RunArgs) -> anyhow::Result<()> {
    #[cfg(feature = "web")]
    if args.is_web() {
        return run_web(args);
    }

    let cargo_args = args.cargo_args_builder();
    // For native builds, wrap `cargo run`
    cargo::run::command().args(cargo_args).ensure_status()?;

    Ok(())
}
