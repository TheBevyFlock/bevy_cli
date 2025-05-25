use crate::external_cli::wasm_opt;
use anyhow::Context as _;
use tracing::info;

use crate::{
    bin_target::BinTarget,
    build::args::{BuildArgs, BuildSubcommands},
    external_cli::{
        cargo::{self, metadata::Metadata},
        wasm_bindgen,
    },
    web::{
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
pub fn build_web(
    args: &mut BuildArgs,
    metadata: &Metadata,
    bin_target: &BinTarget,
) -> anyhow::Result<WebBundle> {
    let web_args = args
        .subcommand
        .as_ref()
        .map(|BuildSubcommands::Web(web_args)| web_args);

    let mut profile_args = configure_default_web_profiles(metadata)?;
    // `--config` args are resolved from left to right,
    // so the default configuration needs to come before the user args
    profile_args.append(&mut args.cargo_args.common_args.config);
    args.cargo_args.common_args.config = profile_args;

    let cargo_args = args.cargo_args_builder();

    info!("compiling to WebAssembly...");

    cargo::build::command()
        // Wasm targets are not installed by default
        .maybe_require_target(args.target())
        .args(cargo_args)
        .env("RUSTFLAGS", args.cargo_args.common_args.rustflags.clone())
        .ensure_status(args.auto_install())?;

    info!("bundling JavaScript bindings...");
    wasm_bindgen::bundle(metadata, bin_target, args.auto_install())?;

    if args.use_wasm_opt() {
        wasm_opt::optimize_path(bin_target, args.auto_install())?;
    }

    let web_bundle = create_web_bundle(
        metadata,
        args.profile(),
        bin_target,
        web_args.is_some_and(|web_args| web_args.create_packed_bundle),
    )
    .context("failed to create web bundle")?;

    if let WebBundle::Packed(PackedBundle { path }) = &web_bundle {
        info!("created bundle at file://{}", path.display());
    }

    Ok(web_bundle)
}
