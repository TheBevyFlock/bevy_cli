use crate::{
    external_cli::{cargo, rustup, wasm_bindgen},
    manifest::package_name,
};

pub use self::args::BuildArgs;

mod args;

pub fn build(args: &BuildArgs) -> anyhow::Result<()> {
    if args.is_web() {
        ensure_web_setup()?;
    }

    let cargo_args = args.cargo_args_builder();

    if args.is_web() {
        println!("Building for WASM...");
        cargo::build::command().args(cargo_args).status()?;

        println!("Bundling for the web...");
        wasm_bindgen::bundle(&package_name()?, args.profile())
            .expect("Failed to bundle for the web");
    } else {
        cargo::build::command().args(cargo_args).status()?;
    }

    Ok(())
}

pub(crate) fn ensure_web_setup() -> anyhow::Result<()> {
    // `wasm32-unknown-unknown` compilation target
    rustup::install_target_if_needed("wasm32-unknown-unknown")?;
    // `wasm-bindgen-cli` for bundling
    cargo::install::if_needed(wasm_bindgen::PROGRAM, wasm_bindgen::PACKAGE, true, false)?;

    Ok(())
}
