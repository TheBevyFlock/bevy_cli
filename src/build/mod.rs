use args::BuildArgs;

#[cfg(feature = "rustup")]
use crate::external_cli::rustup;

#[cfg(feature = "web")]
use crate::web::build::build_web;
use crate::{
    external_cli::{cargo, wasm_bindgen, CommandHelpers},
    run::select_run_binary,
    web::{
        bundle::{create_web_bundle, PackedBundle, WebBundle},
        profiles::configure_default_web_profiles,
    },
};

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
