use std::sync::RwLock;

use rustc_interface::Config;
use rustc_lint::Level;
use rustc_session::{config::Input, utils::was_invoked_from_cargo};
use serde::Deserialize;
use toml::{Table, Value};

static LINTER_CONFIG: RwLock<Option<Table>> = RwLock::new(None);

pub fn load_config(compiler_config: &mut Config) {
    let mut linter_config = LINTER_CONFIG.write().unwrap();

    // Clear the config, if any exists. This prevents old values from leaking into new sessions,
    // in the case we cannot load the configuration.
    *linter_config = None;

    if !was_invoked_from_cargo() {
        return;
    }

    // Load the linter configuration for both the workspace and crate `Cargo.toml`s.
    let workspace_config = load_cargo_manifest(compiler_config, true)
        .and_then(|manifest| deserialize_linter_config(manifest, true));

    let crate_config = load_cargo_manifest(compiler_config, false)
        .and_then(|manifest| deserialize_linter_config(manifest, false));

    let config = match (workspace_config, crate_config) {
        // If only one of the configs are specified, just use that.
        (Some(config), None) | (None, Some(config)) => config,
        // If both configs are specified, merge them.
        (Some(workspace_config), Some(crate_config)) => {
            merge_linter_configs(workspace_config, crate_config)
        }
        // If neither of the configs are specified, exit.
        (None, None) => return,
    };

    insert_lint_levels(compiler_config, &config);

    *linter_config = Some(config);
}

/// Returns the contents of `Cargo.toml` associated with the crate being compiled.
///
/// This will return [`None`] if `Cargo.toml` cannot be located or read. If `workspace` is true,
/// this will instead return the workspace `Cargo.toml`.
fn load_cargo_manifest(compiler_config: &Config, workspace: bool) -> Option<String> {
    let Input::File(ref input_path) = compiler_config.input else {
        // A string was passed directly to the compiler, not a file, so we cannot locate the Cargo
        // project.
        return None;
    };

    let manifest_path = crate::utils::cargo::locate_manifest(input_path, workspace).ok()?;

    std::fs::read_to_string(manifest_path).ok()
}

/// Returns the [`Table`] representing `[package.metadata.bevy_lint]` given the string contents of
/// `Cargo.toml`.
///
/// This will return [`None`] if [`toml`] cannot deserialize the `manifest`, or if
/// `[package.metadata.bevy_lint]` is not specified. If `workspace` is true, this will instead
/// return the [`Table`] for `[workspace.metadata.bevy_lint]`.
fn deserialize_linter_config(manifest: String, workspace: bool) -> Option<Table> {
    /// Represents `Cargo.toml` in the following format:
    ///
    /// ```toml
    /// [package.metadata.bevy_lint]
    /// lint_name = "level"
    /// other_lint_name = { level = "level", foo = 8, bar = false }
    ///
    /// [workspace.metadata.bevy_lint]
    /// yet_another_lint_name = "level"
    /// ```
    #[derive(Deserialize)]
    struct Manifest {
        package: Option<PackageOrWorkspace>,
        workspace: Option<PackageOrWorkspace>,
    }

    #[derive(Deserialize)]
    struct PackageOrWorkspace {
        metadata: Option<Metadata>,
    }

    #[derive(Deserialize)]
    struct Metadata {
        bevy_lint: Option<Table>,
    }

    toml::from_str::<Manifest>(&manifest)
        .ok()
        .and_then(|manifest| {
            if workspace {
                manifest.workspace
            } else {
                manifest.package
            }
        })
        .and_then(|package_or_workspace| package_or_workspace.metadata)
        .and_then(|metadata| metadata.bevy_lint)
}

/// Merges the [`Table`]s for the workspace and crate linter configs together.
///
/// The crate `Cargo.toml` takes precedence, so its configuration will overwrite the workspace's
/// configuration.
fn merge_linter_configs(mut workspace_config: Table, crate_config: Table) -> Table {
    for (lint_name, lint_config) in crate_config {
        // `Map::insert()` overwrite an existing value with a new one, if one already existed.
        workspace_config.insert(lint_name, lint_config);
    }

    workspace_config
}

/// Informs the compiler of `Cargo.toml` lint level configuration.
///
/// This function inserts `--warn`, `--allow`, `--deny`, and more flags into the compiler
/// [`Config`] to set lint levels. For example:
///
/// ```toml
/// [package.metadata.bevy_lint]
/// # Inserts `--warn bevy::missing_reflect`.
/// missing_reflect = "warn"
/// # Inserts `--allow bevy::insert_event_resource`.
/// insert_event_resource = { level = "allow" }
/// ```
fn insert_lint_levels(compiler_config: &mut Config, linter_config: &Table) {
    for (lint_name, lint_config) in linter_config {
        let level = match lint_config {
            // The format of `lint_name = "level"`.
            Value::String(s) => Level::from_str(s),
            // The format of `lint_name = { level = "level" }`.
            Value::Table(table) => table
                .get("level")
                .and_then(Value::as_str)
                .and_then(Level::from_str),
            _ => None,
        };

        if let Some(level) = level {
            compiler_config
                .opts
                .lint_opts
                .push((format!("bevy::{lint_name}"), level));
        }
    }
}
