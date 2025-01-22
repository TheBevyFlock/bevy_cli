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

    let Some(manifest) = load_cargo_manifest(compiler_config) else {
        return;
    };

    let Some(config) = deserialize_linter_config(manifest) else {
        return;
    };

    insert_lint_levels(compiler_config, &config);

    *linter_config = Some(config);
}

pub fn load_lint_config<'de, T: Deserialize<'de>>(lint_name: &str) -> Option<T> {
    let linter_config = LINTER_CONFIG.read().unwrap();

    if let Some(ref linter_config) = *linter_config
        && let Some(lint_config) = linter_config.get(lint_name)
    {
        T::deserialize(lint_config.clone()).ok()
    } else {
        None
    }
}

/// Returns the contents of `Cargo.toml` associated with the crate being compiled.
///
/// This will return [`None`] if `Cargo.toml` cannot be located or read.
fn load_cargo_manifest(compiler_config: &Config) -> Option<String> {
    let Input::File(ref input_path) = compiler_config.input else {
        // A string was passed directly to the compiler, not a file, so we cannot locate the Cargo
        // project.
        return None;
    };

    let manifest_path = crate::utils::cargo::locate_manifest(input_path, false).ok()?;

    std::fs::read_to_string(manifest_path).ok()
}

/// Returns the [`Table`] representing `[package.metadata.bevy_lint]` given the string contents of
/// `Cargo.toml`.
///
/// This will return [`None`] if [`toml`] cannot deserialize the `manifest`.
fn deserialize_linter_config(manifest: String) -> Option<Table> {
    /// Represents `Cargo.toml` in the following format:
    ///
    /// ```toml
    /// [package.metadata.bevy_lint]
    /// lint_name = "level"
    /// other_lint_name = { level = "level", foo = 8, bar = false }
    /// ```
    #[derive(Deserialize)]
    struct Manifest {
        package: Package,
    }

    #[derive(Deserialize)]
    struct Package {
        metadata: Metadata,
    }

    #[derive(Deserialize)]
    struct Metadata {
        bevy_lint: Table,
    }

    toml::from_str::<Manifest>(&manifest)
        .map(|manifest| manifest.package.metadata.bevy_lint)
        .ok()
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
