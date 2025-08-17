use semver::{Version, VersionReq};

use crate::{commands::build::BuildArgs, external_cli::cargo};

/// Apply the web backend to getrandom.
///
/// This is a consistent pain point for users, so we try to automate it.
///
/// There's two things you need to do:
///
/// 1. Enabling the corresponding feature in the `getrandom` dependency
/// 2. Setting the `--cfg getrandom_backend="wasm_js"` RUSTFLAG
///
/// This function can handle both v2 and v3 dependencies.
///
/// If `getrandom` is not a dependency, nothing happens.
pub fn apply_getrandom_backend(args: &mut BuildArgs, target: &str) -> anyhow::Result<bool> {
    // Obtaining the metadata again to filter dependencies and their features by the used target
    let metadata = cargo::metadata::metadata_with_args(["--filter-platform", target])?;

    let getrandom_packages = metadata
        .packages
        .iter()
        .filter(|pkg| pkg.name.as_str() == "getrandom")
        .collect::<Vec<_>>();

    if getrandom_packages.is_empty() {
        // Nothing to do when `getrandom` isn't used
        return Ok(false);
    }

    let mut backend_applied = false;

    // The package allows us to find the correct version of the dependency
    let v2 = getrandom_packages
        .iter()
        .find(|pkg| VersionReq::parse("^0.2").unwrap().matches(&pkg.version));

    // getrandom v2 needs the `js` feature enabled
    if let Some(v2) = v2 {
        // The resolved dependency includes the features that are enabled
        let dep = metadata
            .resolve
            .as_ref()
            .and_then(|resolve| resolve.nodes.iter().find(|node| node.id == v2.id));

        let v2_feature = "js";

        if let Some(dep) = dep
            && !dep.features.iter().any(|feature| **feature == v2_feature)
        {
            backend_applied = true;

            add_feature_config(
                &mut args.cargo_args.common_args.config,
                &v2.version,
                v2_feature,
            );
        }
    }

    // getrandom v3 needs the `wasm_js` feature enabled
    // and the `--cfg getrandom_backend="wasm_js"` RUSTFLAG must be set
    let v3 = getrandom_packages
        .iter()
        .find(|pkg| VersionReq::parse("^0.3").unwrap().matches(&pkg.version));

    if let Some(v3) = v3 {
        let dep = metadata
            .resolve
            .as_ref()
            .and_then(|resolve| resolve.nodes.iter().find(|node| node.id == v3.id));

        let v3_feature = "wasm_js";

        if let Some(dep) = dep
            && !dep.features.iter().any(|feature| **feature == v3_feature)
        {
            backend_applied = true;

            add_feature_config(
                &mut args.cargo_args.common_args.config,
                &v3.version,
                v3_feature,
            );
        }

        let mut rustflags = args
            .cargo_args
            .common_args
            .rustflags
            .clone()
            .unwrap_or_default();

        if !rustflags.contains("getrandom_backend") {
            backend_applied = true;
            rustflags += " --cfg getrandom_backend=\"wasm_js\"";
            args.cargo_args.common_args.rustflags = Some(rustflags);
        }
    }

    Ok(backend_applied)
}

/// Adds the feature configuration for the `getrandom` dependency
fn add_feature_config(config: &mut Vec<String>, version: &Version, feature: &str) {
    // Distinguish entries by version to allow multiple versions to be configured
    let table = format!("dependencies.getrandom_{}{}", version.major, version.minor);
    // The config arg doesn't support inline tables, so each entries must be configured separately
    config.push(format!(r#"{table}.package="getrandom""#));
    config.push(format!(r#"{table}.version="{}""#, version));
    config.push(format!(r#"{table}.features=["{feature}"]"#));
}
