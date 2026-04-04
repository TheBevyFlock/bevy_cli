use anyhow::Context as _;
use cargo_metadata::Metadata;
use tracing::info;

use super::bundle::WebBundle;
use crate::{
    bin_target::select_run_binary,
    commands::build::{BuildArgs, BuildSubcommands},
    external_cli::{cargo, wasm_bindgen, wasm_opt},
    web::{
        bundle::{PackedBundle, create_web_bundle},
        getrandom::{apply_getrandom_backend, getrandom_web_feature_config},
        profiles::configure_default_web_profiles,
    },
};

/// Build the Bevy app for use in the browser.
///
/// The following steps will be performed:
/// - Installing required tooling
/// - Setting up default web compilation profiles
/// - Compiling to Wasm
/// - Optimizing the Wasm binary (in release mode)
/// - Creating JavaScript bindings
/// - Creating a bundled folder (if requested)
pub fn build_web(args: &mut BuildArgs, metadata: &Metadata) -> anyhow::Result<WebBundle> {
    let bin_target = select_run_binary(
        metadata,
        args.cargo_args.package_args.package.as_deref(),
        args.cargo_args.target_args.bin.as_deref(),
        args.cargo_args.target_args.example.as_deref(),
        args.target().as_deref(),
        args.profile(),
    )?;

    let mut profile_args = configure_default_web_profiles(metadata)?;
    // `--config` args are resolved from left to right,
    // so the default configuration needs to come before the user args
    profile_args.append(&mut args.cargo_args.common_args.config);
    args.cargo_args.common_args.config = profile_args;

    // Apply the `getrandom` web backend if necessary
    if apply_getrandom_backend(metadata, &mut args.cargo_args.common_args) {
        info!("automatically configuring `getrandom` web backend");
    }

    #[cfg(feature = "unstable")]
    support_multi_threading(args);

    let cargo_args = args.cargo_args_builder();

    info!("compiling to WebAssembly...");

    cargo::build::command()
        // Wasm targets are not installed by default
        .maybe_require_target(args.target())
        .args(cargo_args)
        .env("RUSTFLAGS", args.rustflags())
        .ensure_status(args.auto_install())
        .inspect_err(|_| {
            // If the build failed, check if the user has configured `getrandom` correctly
            if let Some(target) = args.target()
                && let Ok(Some(feature_config)) = getrandom_web_feature_config(&target)
            {
                tracing::warn!(
                    "You have to enable the `getrandom` web feature in your Cargo.toml:\n\n{feature_config}"
                );
            }
        })?;

    info!("bundling JavaScript bindings...");
    wasm_bindgen::bundle(metadata, &bin_target, args.auto_install())?;
    wasm_opt::optimize_path(&bin_target, args.auto_install(), &args.wasm_opt_args())?;

    let web_args = args
        .subcommand
        .as_ref()
        .map(|BuildSubcommands::Web(web_args)| web_args);

    let web_bundle = create_web_bundle(
        metadata,
        args.profile(),
        &bin_target,
        web_args.is_some_and(|web_args| web_args.create_packed_bundle),
    )
    .context("failed to create web bundle")?;

    if let WebBundle::Packed(PackedBundle { path }) = &web_bundle {
        info!("created bundle at file://{}", path.display());
    }

    Ok(web_bundle)
}

/// Add multi-threading support for the Wasm binary if enabled.
///
/// Requires nightly Rust and the `unstable` feature to be enabled.
#[cfg(feature = "unstable")]
fn support_multi_threading(args: &mut BuildArgs) {
    if !matches!(
        &args.subcommand,
        Some(BuildSubcommands::Web(web_args)) if web_args.unstable.web_multi_threading()
    ) {
        return;
    }

    // Rust's default Wasm target does not support multi-threading primitives out of the box
    // They need to be enabled manually
    let multi_threading_flags =
        crate::web::unstable::UnstableWebArgs::MULTITHREADING_RUSTFLAGS.join(" ");

    if let Some(rustflags) = args.cargo_args.common_args.rustflags.as_mut() {
        *rustflags += " ";
        *rustflags += &multi_threading_flags;
    } else {
        args.cargo_args.common_args.rustflags = Some(multi_threading_flags);
    }

    // The std needs to be rebuilt with Wasm multi-threading support
    // But only for targets that actually include std
    if args
        .target()
        .is_some_and(|target| &target == "wasm32-unknown-unknown")
    {
        // This requires nightly Rust
        args.cargo_args
            .common_args
            .unstable_flags
            .push("build-std=std,panic_abort".to_owned());
    }
}
