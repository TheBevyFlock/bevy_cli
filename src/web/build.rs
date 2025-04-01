use anyhow::Context as _;
use tracing::info;

#[cfg(feature = "wasm-opt")]
use crate::external_cli::wasm_opt;
use crate::{
    build::args::{BuildArgs, BuildSubcommands},
    external_cli::{cargo, rustup, wasm_bindgen},
    web::{
        bin_target::select_run_binary,
        bundle::{PackedBundle, create_web_bundle},
        profiles::configure_default_web_profiles,
    },
};

use super::bundle::WebBundle;

/// Build the Bevy app for use in the browser.
///
/// The following steps will be performed:
/// - Installing required tooling
/// - Setting up default web compilation profiles
/// - Compiling to Wasm
/// - Optimizing the Wasm binary (in release mode)
/// - Creating JavaScript bindings
/// - Creating a bundled folder (if requested)
pub fn build_web(args: &mut BuildArgs) -> anyhow::Result<WebBundle> {
    let Some(BuildSubcommands::Web(web_args)) = &args.subcommand else {
        anyhow::bail!("tried to build for the web without matching arguments");
    };

    ensure_web_setup(args.skip_prompts)?;

    let metadata = cargo::metadata::metadata_with_args(["--no-deps"])?;
    let bin_target = select_run_binary(
        &metadata,
        args.cargo_args.package_args.package.as_deref(),
        args.cargo_args.target_args.bin.as_deref(),
        args.cargo_args.target_args.example.as_deref(),
        args.target().as_deref(),
        args.profile(),
    )?;

    let mut profile_args = configure_default_web_profiles(&metadata)?;
    // `--config` args are resolved from left to right,
    // so the default configuration needs to come before the user args
    profile_args.append(&mut args.cargo_args.common_args.config);
    args.cargo_args.common_args.config = profile_args;

    let cargo_args = args.cargo_args_builder();

    info!("Compiling to WebAssembly...");
    cargo::build::command().args(cargo_args).ensure_status()?;

    info!("Bundling JavaScript bindings...");
    wasm_bindgen::bundle(&bin_target)?;

    #[cfg(feature = "wasm-opt")]
    if args.is_release() {
        wasm_opt::optimize_path(&bin_target)?;
    }

    let web_bundle = create_web_bundle(
        &metadata,
        args.profile(),
        &bin_target,
        web_args.create_packed_bundle,
    )
    .context("Failed to create web bundle")?;

    if let WebBundle::Packed(PackedBundle { path }) = &web_bundle {
        info!("Created bundle at file://{}", path.display());
    }

    Ok(web_bundle)
}

pub(crate) fn ensure_web_setup(skip_prompts: bool) -> anyhow::Result<()> {
    // The resolved dependency graph is needed to ensure the `wasm-bindgen-cli` version matches
    // exactly the `wasm-bindgen` version
    let metadata = cargo::metadata::metadata()?;

    let wasm_bindgen_version = metadata
        .packages
        .iter()
        .find(|package| package.name == "wasm-bindgen")
        .map(|package| package.version.to_string())
        .ok_or_else(|| anyhow::anyhow!("Failed to find wasm-bindgen"))?;

    // `wasm32-unknown-unknown` compilation target
    rustup::install_target_if_needed("wasm32-unknown-unknown", skip_prompts)?;
    // `wasm-bindgen-cli` for bundling
    cargo::install::if_needed(
        wasm_bindgen::PROGRAM,
        wasm_bindgen::PACKAGE,
        Some(&wasm_bindgen_version),
        skip_prompts,
    )?;

    // `wasm-opt` for optimizing wasm files
    #[cfg(feature = "wasm-opt")]
    cargo::install::if_needed(wasm_opt::PACKAGE, wasm_opt::PROGRAM, None, skip_prompts)?;

    Ok(())
}
