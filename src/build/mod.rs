use anyhow::bail;
use args::{BuildArgs, BuildSubcommands};

use crate::{
    external_cli::{cargo, rustup, wasm_bindgen, CommandHelpers},
    run::{select_run_binary, BinTarget},
    web::{
        bundle::{create_web_bundle, PackedBundle, WebBundle},
        profiles::configure_default_web_profiles,
    },
};

pub mod args;

pub fn build(args: &BuildArgs) -> anyhow::Result<()> {
    if args.is_web() {
        build_web(args)?;
    } else {
        let cargo_args = args.cargo_args_builder();
        cargo::build::command().args(cargo_args).ensure_status()?;
    }

    Ok(())
}

/// Build the Bevy app for use in the browser.
///
/// The following steps will be performed:
/// - Installing required tooling
/// - Setting up default web compilation profiles
/// - Compiling to Wasm
/// - Optimizing the Wasm binary (in release mode)
/// - Creating JavaScript bindings
/// - Creating a bundled folder (if requested)
fn build_web(args: &BuildArgs) -> anyhow::Result<BinTarget> {
    let Some(BuildSubcommands::Web(web_args)) = &args.subcommand else {
        bail!("tried to build for the web without matching arguments");
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

    let cargo_args = args
        .cargo_args_builder()
        .append(configure_default_web_profiles(&metadata)?);

    println!("Compiling to WebAssembly...");
    cargo::build::command().args(cargo_args).ensure_status()?;

    println!("Bundling JavaScript bindings...");
    wasm_bindgen::bundle(&bin_target)?;

    #[cfg(feature = "wasm-opt")]
    if args.is_release() {
        crate::web::wasm_opt::optimize_bin(&bin_target)?;
    }

    if web_args.create_packed_bundle {
        let web_bundle = create_web_bundle(&metadata, args.profile(), &bin_target, true)?;

        if let WebBundle::Packed(PackedBundle { path }) = &web_bundle {
            println!("Created bundle at file://{}", path.display());
        }
    }

    Ok(bin_target)
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
        false,
    )?;

    Ok(())
}
