use cargo_metadata::{Metadata, Package};
use semver::VersionReq;

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
/// This function only does the second part, as the first one can't be automated reliably
/// without modifying the user's `Cargo.toml`.
///
/// When `true` is returned, the rustflag has been configured in the args.
pub fn apply_getrandom_backend(metadata: &Metadata, args: &mut BuildArgs) -> bool {
    let getrandom = getrandom_packages(metadata);

    if getrandom.v3_packages.is_empty() {
        // Nothing to do when `getrandom` isn't used
        return false;
    }

    let mut rustflags = args
        .cargo_args
        .common_args
        .rustflags
        .clone()
        .unwrap_or_default();

    if rustflags.contains("getrandom_backend") {
        // The user has already set a backend, so we don't override it
        return false;
    }

    // Add the backend configuration
    rustflags += " --cfg getrandom_backend=\"wasm_js\"";
    args.cargo_args.common_args.rustflags = Some(rustflags);

    true
}

/// Generate a Cargo.toml configuration snippet to set the web features for `getrandom`.
/// If `getrandom` isn't used, or the features are already enabled, `Ok(None)` is returned.
pub fn getrandom_web_feature_config(target: &str) -> anyhow::Result<Option<String>> {
    // Obtaining the metadata again to filter dependencies and their features by the used target
    let metadata = cargo::metadata::metadata_with_args(["--filter-platform", target])?;

    let getrandom = getrandom_packages(&metadata);

    let needs_v2_fix = needs_feature_configuration(&metadata, &getrandom.v2_packages, "js");
    let needs_v3_fix = needs_feature_configuration(&metadata, &getrandom.v3_packages, "wasm_js");

    if !needs_v2_fix && !needs_v3_fix {
        return Ok(None);
    }

    let multiple_entries = needs_v2_fix && needs_v3_fix;

    // Since the user might want to build for multiple platforms,
    // only configure the features when targeting Wasm
    let mut getrandom_config = r#"# Enable the `getrandom` web backend
[target.'cfg(all(target_family = "wasm", any(target_os = "unknown", target_os = "none")))'.dependencies]"#.to_owned();

    if needs_v2_fix {
        let v2 = VersionReq::parse("0.2").unwrap();

        getrandom_config += "\n";
        getrandom_config += &feature_entry(&v2, "js", multiple_entries);
    }

    if needs_v3_fix {
        let v3 = VersionReq::parse("0.3").unwrap();
        getrandom_config += "\n";
        getrandom_config += &feature_entry(&v3, "wasm_js", multiple_entries);
    }

    Ok(Some(getrandom_config))
}

#[derive(Debug)]
struct GetRandom<'p> {
    v2_packages: Vec<&'p Package>,
    v3_packages: Vec<&'p Package>,
}

/// Determine which versions of `getrandom` are used in the project.
fn getrandom_packages(metadata: &Metadata) -> GetRandom<'_> {
    let v2 = VersionReq::parse("^0.2").unwrap();
    let v3 = VersionReq::parse("^0.3").unwrap();

    let mut v2_packages = Vec::new();
    let mut v3_packages = Vec::new();

    metadata
        .packages
        .iter()
        .filter(|pkg| pkg.name.as_str() == "getrandom")
        .for_each(|package| {
            if v2.matches(&package.version) {
                v2_packages.push(package);
            } else if v3.matches(&package.version) {
                v3_packages.push(package);
            }
        });

    GetRandom {
        v2_packages,
        v3_packages,
    }
}

/// Determine if the given feature needs to be enabled for any of the packages.
/// The metadata must include dependencies and be filtered to the correct target platform.
fn needs_feature_configuration(
    metadata: &Metadata,
    packages: &Vec<&Package>,
    feature: &str,
) -> bool {
    if packages.is_empty() {
        return false;
    }

    packages
        .iter()
        .any(|package| !has_feature(metadata, package, feature))
}

/// Check if a specific feature is enabled for a package.
/// The metadata must include dependencies and be filtered to the correct target platform.
fn has_feature(metadata: &Metadata, package: &Package, feature: &str) -> bool {
    metadata
        .resolve
        .as_ref()
        .and_then(|resolve| {
            resolve.nodes.iter().find(|node| {
                node.id == package.id
                    && node
                        .features
                        .iter()
                        .any(|dep_feature| **dep_feature == feature)
            })
        })
        .is_some()
}

/// Generate a dependency entry for the Cargo.toml snippet, enabling the given feature.
fn feature_entry(version: &VersionReq, feature: &str, multiple_entries: bool) -> String {
    let package = "getrandom";

    if multiple_entries {
        // If multiple fixes with different versions are needed,
        // the name for each entry must be unique
        format!(
            r#"{package}_{} = {{ version = "{version}", features = ["{feature}"], package = "{package}" }}"#,
            version.to_string().replace(['^', '.', '='], "")
        )
    } else {
        format!(r#"{package} = {{ version = "{version}", features = ["{feature}"] }}"#)
    }
}
