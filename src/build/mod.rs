use args::BuildArgs;

use crate::external_cli::cargo;
#[cfg(feature = "web")]
use crate::web::build::build_web;

pub mod args;

pub fn build(args: &mut BuildArgs) -> anyhow::Result<()> {
    #[cfg(feature = "web")]
    if args.is_web() {
        build_web(args)?;
        return Ok(());
    }

    let cargo_args = args.cargo_args_builder();
    cargo::build::command().args(cargo_args).ensure_status()?;

    Ok(())
}
