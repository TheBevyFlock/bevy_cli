mod args;

pub use self::args::PatchArgs;

use crate::external_cli::cargo::{self, metadata::DependencyKind};
use anyhow::bail;
use args::GitRevisionArgs;

/// A list of crates that `bevy_internal` does not depend on, and as such need to be hard-coded.
const INJECTED_BEVY_CRATES: &[&str] = &[
    // `bevy_internal` cannot depend on `bevy` because `bevy` depends on `bevy_internal`.
    "bevy",
    // `bevy_internal` cannot depend on itself.
    "bevy_internal",
    // `bevy` depends on `bevy_dylib` directly, not `bevy_internal`.
    "bevy_dylib",
];

pub fn patch(args: &PatchArgs) -> anyhow::Result<()> {
    let metadata = cargo::metadata::metadata()?;

    // Find `bevy_internal` within this workspace's list of packages. (This list contains
    // dependencies of crates, since we didn't specify `--no-deps`.) `bevy_internal` depends on
    // almost all official Bevy crates, excluding `bevy`, `bevy_dylib`, and `bevy_internal` itself.
    let Some(bevy_internal) = metadata
        .packages
        .into_iter()
        .find(|p| p.name == "bevy_internal")
    else {
        bail!(
            "`bevy_internal` cannot be found in dependencies. Is Bevy installed as a dependency?"
        );
    };

    let injected_bevy_crates = INJECTED_BEVY_CRATES.into_iter().map(|s| s.to_string());

    let official_bevy_crates = bevy_internal
        .dependencies
        .into_iter()
        // Skip dev-dependencies and build-dependencies.
        .filter(|d| matches!(d.kind, DependencyKind::Null))
        // While `bevy_internal` doesn't directly depend on any non-Bevy crates currently, that may
        // change in the future. This is a future-proof, in case a crate like `cfg_if` is added.
        .filter(|d| d.name.starts_with("bevy_"))
        .map(|d| d.name);

    // Generate a (potentially empty) string containing the Git repository revision specification.
    // For example, if `--branch patch-1` was specified, this would be `, branch = "patch-1"`.
    let revision_fragment = git_revision_fragment(&args.git_revision_args);

    println!("[patch.crates-io]");

    for d in injected_bevy_crates.chain(official_bevy_crates) {
        println!("{} = {{ git = \"{}\"{} }}", d, args.git, revision_fragment);
    }

    Ok(())
}

/// Returns a string fragment leading after `git = "<git>"` in a dependency patch specification.
fn git_revision_fragment(revision_args: &GitRevisionArgs) -> String {
    // `branch`, `tag`, and `rev` are exclusive to each other in the CLI. If we find `Some` for
    // one, we can assume that the other two are `None`.
    match revision_args {
        // If no revision is specified, use the default `{ git = "<git>" }`.
        GitRevisionArgs {
            branch: None,
            tag: None,
            rev: None,
        } => String::new(),

        // If `--branch` is specified, use `{ git = "<git>", branch = "<branch>" }`.
        GitRevisionArgs {
            branch: Some(branch),
            ..
        } => format!(", branch = \"{branch}\""),

        // If `--tag` is specified, use `{ git = "<git>", tag = "<tag>" }`.
        GitRevisionArgs { tag: Some(tag), .. } => format!(", tag = \"{tag}\""),

        // If `--rev` is specified, use `{ git = "<git>", rev = "<rev>" }`.
        GitRevisionArgs { rev: Some(rev), .. } => format!(", rev = \"{rev}\""),
    }
}
