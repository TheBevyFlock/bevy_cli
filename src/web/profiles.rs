use std::{collections::HashMap, fs};

use anyhow::Context as _;
use toml_edit::DocumentMut;

use crate::{
    external_cli::{arg_builder::ArgBuilder, cargo::metadata::Metadata},
    run::BinTarget,
};

/// Create `--config` args to configure the default profiles to use when compiling for the web.
pub(crate) fn configure_default_web_profiles(
    metadata: &Metadata,
    bin_target: &BinTarget,
) -> anyhow::Result<ArgBuilder> {
    let package_manifest = fs::read_to_string(&bin_target.package.manifest_path)
        .context("failed to read package manifest")?
        .parse::<DocumentMut>()
        .context("failed to parse package manifest")?;
    let workspace_manifest = metadata.workspace_root.as_ref().and_then(|root| {
        fs::read_to_string(root.join("Cargo.toml"))
            .ok()?
            .parse::<DocumentMut>()
            .ok()
    });

    let mut args = ArgBuilder::new();

    if !is_profile_defined(&package_manifest, workspace_manifest.as_ref(), "web") {
        args = args.append(configure_web_profile());
    }

    if !is_profile_defined(
        &package_manifest,
        workspace_manifest.as_ref(),
        "web-release",
    ) {
        args = args.append(configure_web_release_profile());
    }

    Ok(args)
}

/// Check whether the user defined the given profile either in the package or workspace.
fn is_profile_defined(
    package_manifest: &DocumentMut,
    workspace_manifest: Option<&DocumentMut>,
    profile: &str,
) -> bool {
    is_profile_defined_in_manifest(package_manifest, profile)
        || workspace_manifest
            .is_some_and(|manifest| is_profile_defined_in_manifest(manifest, profile))
}

fn is_profile_defined_in_manifest(manifest: &DocumentMut, profile: &str) -> bool {
    manifest
        .get("profile")
        .is_some_and(|profiles| profiles.get(profile).is_some())
}

/// Configure the default profile for web debug builds.
///
/// It is optimized for fast iteration speeds.
fn configure_web_profile() -> ArgBuilder {
    configure_profile("web", "dev", HashMap::new())
}

/// Configure the default profile for web release builds.
///
/// It is optimized both for run time performance and loading times.
fn configure_web_release_profile() -> ArgBuilder {
    let config = HashMap::from_iter([
        // Optimize for size, greatly reducing loading times
        ("opt-level", "s"),
        // Remove debug information, reducing file size further
        ("strip", "debuginfo"),
    ]);
    configure_profile("web-release", "release", config)
}

/// Create `--config` args for `cargo` to configure a new compilation profile.
///
/// Equivalent to a `Cargo.toml` like this:
///
/// ```toml
/// [profile.{profile}]
/// inherits = "{inherits}"
/// # config
/// key = "value"
/// ```
fn configure_profile(profile: &str, inherits: &str, config: HashMap<&str, &str>) -> ArgBuilder {
    let mut args = ArgBuilder::new().add_with_value(
        "--config",
        format!(r#"profile.{profile}.inherits="{inherits}""#),
    );

    for (key, value) in config {
        args = args.add_with_value("--config", format!(r#"profile.{profile}.{key}="{value}""#));
    }

    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_detect_defined_profile() {
        let manifest = r#"
        [profile.web]
        inherits = "dev"
        "#
        .parse()
        .unwrap();

        assert!(is_profile_defined_in_manifest(&manifest, "web"));
    }

    #[test]
    fn should_detect_missing_profile() {
        let manifest = r#"
        [profile.foo]
        inherits = "dev"
        "#
        .parse()
        .unwrap();

        assert!(!is_profile_defined_in_manifest(&manifest, "web"));
    }
}
