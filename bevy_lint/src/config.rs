//! The main mechanism for loading the linter's configuration from `Cargo.toml`.
//!
//! The main entrypoint for configuration loading is [`load_config()`], which then stores its
//! results in the [`LINTER_CONFIG`] static.

use std::sync::RwLock;

use rustc_interface::Config;
use rustc_lint::Level;
use rustc_session::utils::was_invoked_from_cargo;
use serde::Deserialize;
use serde_json::Value;

static LINTER_CONFIG: RwLock<Option<Value>> = RwLock::new(None);

/// Loads the configuration from the crate and workspace-level `Cargo.toml`s.
///
/// This is the main entrypoint that sets up configuration. It is responsible for reading
/// `Cargo.toml`, merging the configuration together, setting the default lint levels based on that
/// configuration, and storing the config so it can be accessed later on.
pub fn load_config(compiler_config: &mut Config) {
    let mut linter_config = LINTER_CONFIG.write().unwrap();

    // Clear the config, if any exists. This prevents old values from leaking into new sessions,
    // in the case we cannot load the configuration.
    *linter_config = None;

    // Configuration is read from `Cargo.toml`. If we're not being called from Cargo, we should
    // avoid assuming the configuration in `Cargo.toml` is desired by the user.
    if !was_invoked_from_cargo() {
        return;
    }

    let local_cargo_manifest =
        crate::utils::cargo::locate_manifest(&compiler_config.input, false).unwrap();

    // built resolved dependency graph in order to
    // figure out the name of the current package
    // this is needed because the only way to know what package
    // we should read the metadata from, is by using the `resolve.root`
    // with corresponds to the [`cargo_metadata::PackageId`] cargo metadata was run for.
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(local_cargo_manifest)
        .exec()
        .unwrap();

    let root_id = metadata
        .resolve
        .as_ref()
        .and_then(|r| r.root.as_ref())
        .expect("cannot find root package");

    let current_package = metadata
        .packages
        .iter()
        .find(|pkg| pkg.id == *root_id)
        .expect("failed to get the current package");

    let workspace_config = deserialize_linter_config(metadata.workspace_metadata);

    let crate_config = deserialize_linter_config(current_package.metadata.clone());

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

/// Returns the `Value` representing `metadata.bevy_lint` from the provided
/// [`cargo_metadata::Metadata`] contents.
///
/// This function will return [`None`] if the [`cargo_metadata::Metadata`] cannot be deserialized
/// into the expected format, or if `metadata.bevy_lint` is not specified.
fn deserialize_linter_config(metadata: Value) -> Option<Value> {
    #[derive(Deserialize)]
    struct Metadata {
        bevy_lint: Option<Value>,
    }

    serde_json::from_value::<Metadata>(metadata).ok()?.bevy_lint
}

/// Merges the [`Value::Object`]s for the workspace and crate linter configs together.
///
/// The crate `Cargo.toml` takes precedence, so its configuration will overwrite the workspace's
/// configuration.
fn merge_linter_configs(mut workspace_config: Value, crate_config: Value) -> Value {
    match (&mut workspace_config, crate_config) {
        (Value::Object(workspace_obj), Value::Object(crate_obj)) => {
            for (key, value) in crate_obj {
                workspace_obj.insert(key, value);
            }
            workspace_config
        }
        _ => workspace_config,
    }
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
fn insert_lint_levels(compiler_config: &mut Config, linter_config: &Value) {
    let Value::Object(linter_config) = linter_config else {
        return;
    };

    for (lint_name, lint_config) in linter_config {
        let level = match lint_config {
            Value::String(s) => Level::from_str(s),
            Value::Object(obj) => obj
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
