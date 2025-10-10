//! Checks for multiple versions of the `bevy` crate in your project's dependencies.
//!
//! This lint will prevent you from accidentally using multiple versions of the Bevy game engine at
//! the same time by scanning your dependency tree for the `bevy` crate. If your project or its
//! dependencies use different versions of `bevy`, this lint will emit a warning.
//!
//! You may also be interested in [`cargo-deny`], which can detect duplicate dependencies as well,
//! and is far more powerful and configurable.
//!
//! [`cargo-deny`]: https://github.com/EmbarkStudios/cargo-deny
//!
//! # Motivation
//!
//! Cargo allows there to be multiple major versions of a crate in your project's dependency
//! tree[^semver-compatibility]. Although the crates and their types are _named_ the same, they are
//! treated as distinct by the compiler. This can lead to confusing error messages that only appear
//! if you try to mix the types from the two versions of the crate.
//!
//! With Bevy, these errors become particularly easy to encounter when you add a plugin that pulls
//! in a different version of the Bevy engine. (This isn't immediately obvious, however, unless you
//! look at `Cargo.lock` or the plugin's engine compatibility table.)
//!
//! [^semver-compatibility]: The rules for dependency unification and duplication are specified
//!     [here](https://doc.rust-lang.org/cargo/reference/resolver.html#semver-compatibility).
//!
//! # Known Issues
//!
//! This lint only works if a specific version of `bevy` is declared. If a version range is
//! specified, this lint will be skipped. For example:
//!
//! ```toml
//! [dependencies]
//! # This will not be linted, since it is a version range.
//! bevy = ">=0.15"
//! ```
//!
//! # Example
//!
//! ```toml
//! [package]
//! name = "foo"
//! edition = "2024"
//!
//! [dependencies]
//! bevy = "0.15"
//! # This depends on Bevy 0.14, not 0.15! This will cause duplicate versions of the engine.
//! leafwing-input-manager = "0.15"
//! ```
//!
//! Use instead:
//!
//! ```toml
//! [package]
//! name = "foo"
//! edition = "2024"
//!
//! [dependencies]
//! bevy = "0.15"
//! # Update to a newer version of the plugin, which supports Bevy 0.15.
//! leafwing-input-manager = "0.16"
//! ```

use std::{collections::BTreeMap, ops::Range, path::Path, sync::Arc};

use cargo_metadata::{
    Metadata, Resolve,
    semver::{Prerelease, Version, VersionReq},
};
use clippy_utils::{
    diagnostics::{span_lint, span_lint_and_then},
    paths::find_crates,
};
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_lint::LateContext;
use rustc_span::{BytePos, Pos, SourceFile, Span, SyntaxContext};
use serde::Deserialize;
use toml::Spanned;

use crate::{declare_bevy_lint, sym};

declare_bevy_lint! {
    pub(crate) DUPLICATE_BEVY_DEPENDENCIES,
    super::Nursery,
    "multiple versions of the `bevy` crate found",
    @crate_level_only = true,
}

#[derive(Deserialize, Debug)]
struct CargoToml {
    dependencies: BTreeMap<Spanned<String>, Spanned<toml::Value>>,
}

fn toml_span(range: Range<usize>, file: &SourceFile) -> Span {
    Span::new(
        file.start_pos + BytePos::from_usize(range.start),
        file.start_pos + BytePos::from_usize(range.end),
        SyntaxContext::root(),
        None,
    )
}

pub(crate) fn check(cx: &LateContext<'_>, metadata: &Metadata) {
    // Check if there are 2 or more crates named `bevy`.
    if find_crates(cx.tcx, sym::bevy).len() <= 1 {
        return;
    }

    // Load the `Cargo.toml` into the `SourceMap` this is  necessary to get the `Span` of the
    // `Cargo.toml` file.
    if let Ok(file) = cx.tcx.sess.source_map().load_file(Path::new("Cargo.toml"))
        && let Some(src) = file.src.as_deref()
        // Parse the `Cargo.toml` file into a `CargoToml` struct, this helps getting the correct span and not just
        // the root span of the `Cargo.toml` file.
        && let Ok(cargo_toml) = toml::from_str::<CargoToml>(src)
    {
        let local_name = cx.tcx.crate_name(LOCAL_CRATE);

        // get the package name and the corresponding version of `bevy` that they depend on
        let mut bevy_dependents = BTreeMap::default();
        for package in &metadata.packages {
            for dependency in &package.dependencies {
                if dependency.name.as_str() == "bevy"
                    && package.name.as_str() != local_name.as_str()
                {
                    bevy_dependents.insert(package.name.as_str(), dependency.req.clone());
                }
            }
        }

        // If `bevy` is listed as a direct dependency, use its version as the target version for all
        // other crates, and check for any dependents that use a different version.
        // If `bevy` is not listed as a direct dependency, check if multiple versions of `bevy` are
        // resolved. If so, report a single lint error.
        match cargo_toml.dependencies.get("bevy") {
            Some(bevy_cargo) => {
                lint_with_target_version(cx, &cargo_toml, &file, bevy_cargo, &bevy_dependents);
            }

            None => {
                if let Some(resolve) = &metadata.resolve {
                    minimal_lint(cx, &bevy_dependents, resolve);
                }
            }
        }
    }
}

fn lint_with_target_version(
    cx: &LateContext<'_>,
    cargo_toml: &CargoToml,
    file: &Arc<SourceFile>,
    bevy_cargo: &Spanned<toml::Value>,
    bevy_dependents: &BTreeMap<&str, VersionReq>,
) {
    let bevy_cargo_toml_span = toml_span(bevy_cargo.span(), file);

    // Semver only supports checking if a given `VersionReq` matches a `Version` and not if two
    // `VersionReq` can successfully resolve to one `Version`. Therefore we try to parse the
    // `Version` from the `bevy` dependency in the `Cargo.toml` file. This only works if a
    // single version  of `bevy` is specified and not a range.
    let Ok(target_version) = get_version_from_toml(bevy_cargo.as_ref()) else {
        cx.tcx.dcx().span_warn(
            bevy_cargo_toml_span,
            "specified version format is not supported, use a fixed version or disable `bevy::duplicate_bevy_dependencies`",
        );
        return;
    };

    let mismatching_dependencies = bevy_dependents
        .iter()
        .filter(|dependency| !dependency.1.matches(&target_version));

    for mismatching_dependency in mismatching_dependencies {
        if let Some(cargo_toml_reference) = cargo_toml.dependencies.get(*mismatching_dependency.0) {
            span_lint_and_then(
                cx,
                DUPLICATE_BEVY_DEPENDENCIES,
                toml_span(cargo_toml_reference.span(), file),
                DUPLICATE_BEVY_DEPENDENCIES.desc,
                |diag| {
                    diag.span_help(
                        bevy_cargo_toml_span,
                        format!("expected all crates to use `bevy` {target_version}, but `{}` uses `bevy` {}", mismatching_dependency.0, mismatching_dependency.1),
                    );
                },
            );
        }
    }
}

fn minimal_lint(
    cx: &LateContext<'_>,
    bevy_dependents: &BTreeMap<&str, VersionReq>,
    resolved: &Resolve,
) {
    // Examples of the underlying string representation of resolved crates
    // "id": "file:///path/to/my-package#0.1.0",
    // "id": "registry+https://github.com/rust-lang/crates.io-index#bevy@0.9.1",
    let mut resolved_bevy_versions: Vec<&str> = resolved
        .nodes
        .iter()
        .filter_map(|node| {
            // Extract version from local crates
            if node.id.repr.starts_with("file:///") {
                return node.id.repr.split('#').nth(1).map(|version| vec![version]);
            }
            // Extract versions from external crates
            if let Some((id, _)) = node.id.repr.split_once('@')
                && bevy_dependents
                    .keys()
                    .any(|crate_name| id.ends_with(crate_name))
            {
                return Some(
                    node.dependencies
                        .iter()
                        .filter_map(|dep| dep.repr.split_once('@'))
                        .filter_map(|(name, version)| (name.contains("bevy")).then_some(version))
                        .collect(),
                );
            }

            None
        })
        .flatten()
        .collect();

    resolved_bevy_versions.sort_unstable();
    resolved_bevy_versions.dedup();

    if resolved_bevy_versions.len() > 1 {
        span_lint(
            cx,
            DUPLICATE_BEVY_DEPENDENCIES,
            rustc_span::DUMMY_SP,
            "found multiple versions of bevy",
        );
    }
}

/// Extracts the `version` field from a [`toml::Value`] and parses it into a [`Version`]
/// There are two possible formats:
/// 1. A toml-string `<crate> = <version>`
/// 2. A toml-table `<crate> = { version = <version> , ... }`
///
/// Cargo supports specifying version ranges, but [`parse_version()`] can only parse exact versions
/// and not ranges.
fn get_version_from_toml(table: &toml::Value) -> anyhow::Result<Version> {
    match table {
        toml::Value::String(version) => parse_version(version),
        toml::Value::Table(table) => table
            .get("version")
            .and_then(toml::Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("The 'version' field is required."))
            .map(parse_version)?,
        _ => Err(anyhow::anyhow!(
            "Unexpected TOML format: expected a toml-string '<crate> = <version>' or a toml-table with '<crate> = {{ version = <version> }} '"
        )),
    }
}

/// Parse a Version that does not contain any ranges.
/// In contrast to `cargo_metadata::semver::Version::from_str` this also supports versions in the
/// format of `1.1` by just setting the patch level to 0.
fn parse_version(version: &str) -> anyhow::Result<Version> {
    // split at '-' in order to not include the pre release version in the patch if one is present.
    let mut iter = version.split('-');

    // create a copy so we can validate that each part of the semver
    // is a number without consuming the iterator.
    let semver_parts = iter
        .next()
        .ok_or(anyhow::anyhow!("A version string is required"))?
        .split('.')
        .collect::<Vec<&str>>();

    // check if each part of the semver only contains numbers.
    if !semver_parts
        .iter()
        .all(|part| part.chars().all(|c| c.is_ascii_digit()))
    {
        return Err(anyhow::anyhow!("Version ranges are not yet supported"));
    }

    let pre = iter.next();

    let major = semver_parts
        .first()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    let minor = semver_parts
        .get(1)
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    let patch = semver_parts
        .get(2)
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    let mut version = Version::new(major, minor, patch);

    if let Some(pre) = pre {
        version.pre = Prerelease::new(pre).unwrap();
    }
    Ok(version)
}

#[cfg(test)]
mod tests {
    use cargo_metadata::semver::{Prerelease, Version};

    use super::parse_version;

    #[test]
    fn parse_version_req() {
        assert_eq!(Version::new(0, 16, 0), parse_version("0.16").unwrap());
        assert_eq!(Version::new(0, 16, 1), parse_version("0.16.1").unwrap());
        assert_eq!(Version::new(1, 16, 10), parse_version("1.16.10").unwrap());
        let mut version_with_pre = Version::new(0, 16, 0);
        version_with_pre.pre = Prerelease::new("rc.1").unwrap();
        assert_eq!(version_with_pre, parse_version("0.16.0-rc.1").unwrap());
        // This should fail since we specified a version range
        assert!(parse_version("0.*").is_err());
    }
}
