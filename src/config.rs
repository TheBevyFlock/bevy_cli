//! Configuration used by the `bevy_cli`, defined in `Cargo.toml` under `package.metadata.bevy_cli`.
use std::fmt::Display;

use anyhow::{Context, bail};
use serde::Serialize;
use serde_json::{Map, Value};
use tracing::warn;

use crate::external_cli::cargo::metadata::{Metadata, Package};

/// Configuration for the `bevy_cli`.
///
/// Allows customizing:
/// - Target platform
/// - Enabled features
/// - Whether to enable default features
/// - Additional Rust compiler flags
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CliConfig {
    /// The platform to target with the build.
    target: Option<String>,
    /// Additional features that should be enabled.
    features: Vec<String>,
    /// Whether to use default features.
    default_features: Option<bool>,
    /// Additional flags for `rustc`
    rustflags: Vec<String>,
    /// Use `wasm-opt` to optimize wasm binaries.
    wasm_opt: Option<bool>,
}

impl CliConfig {
    /// Returns `true` if the config doesn't change the defaults.
    pub fn is_default(&self) -> bool {
        // Using destructuring to ensure that all fields are considered
        let Self {
            target,
            features,
            default_features,
            rustflags,
            wasm_opt,
        } = self;

        target.is_none()
            && features.is_empty()
            && default_features.is_none()
            && rustflags.is_empty()
            && wasm_opt.is_none()
    }

    /// The platform to target with the build.
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    /// Whether to enable default features.
    ///
    /// Defaults to `true` if not configured otherwise.
    pub fn default_features(&self) -> bool {
        self.default_features.unwrap_or(true)
    }

    /// The features enabled in the config.
    pub fn features(&self) -> &[String] {
        &self.features
    }

    /// The rustflags enabled in the config
    pub fn rustflags(&self) -> Option<String> {
        if self.rustflags.is_empty() {
            return None;
        }
        Some(self.rustflags.clone().join(" "))
    }

    /// Whether to use `wasm-opt`.
    #[cfg(feature = "web")]
    pub fn wasm_opt(&self) -> Option<bool> {
        self.wasm_opt
    }

    /// Determine the Bevy CLI config as defined in the given package.
    pub fn for_package(
        metadata: &Metadata,
        package: &Package,
        is_web: bool,
        is_release: bool,
    ) -> anyhow::Result<Self> {
        let Some(package_metadata) = metadata.packages.iter().find_map(|cur_package| {
            if package == cur_package {
                Some(cur_package.metadata.clone())
            } else {
                None
            }
        }) else {
            return Ok(Self::default());
        };

        let base_metadata = package_metadata.get("bevy_cli");
        Self::merged_from_metadata(base_metadata, is_web, is_release)
    }

    /// Build a config from the `package.metadata.bevy_cli` table.
    ///
    /// It is merged from the platform- and profile-specific configurations.
    fn merged_from_metadata(
        cli_metadata: Option<&Value>,
        is_web: bool,
        is_release: bool,
    ) -> anyhow::Result<Self> {
        let profile = if is_release { "release" } else { "dev" };
        let platform = if is_web { "web" } else { "native" };

        let profile_metadata = cli_metadata.and_then(|metadata| metadata.get(profile));
        let platform_metadata = cli_metadata.and_then(|metadata| metadata.get(platform));
        let platform_profile_metadata =
            platform_metadata.and_then(|metadata| metadata.get(profile));

        // Start with the base config
        let config = Self::from_specific_metadata(cli_metadata)
            .context("failed to parse package.metadata.bevy_cli")?
            // Add the profile-specific config
            .overwrite(
                &Self::from_specific_metadata(profile_metadata).context(format!(
                    "failed to parse package.metadata.bevy_cli.{profile}"
                ))?,
            )
            // Then the platform-specific config
            .overwrite(
                &Self::from_specific_metadata(platform_metadata).context(format!(
                    "failed to parse package.metadata.bevy_cli.{platform}"
                ))?,
            )
            // Finally, the platform-profile combination
            .overwrite(
                &Self::from_specific_metadata(platform_profile_metadata).context(format!(
                    "failed to parse package.metadata.bevy_cli.{platform}.{profile}"
                ))?,
            );

        Ok(config)
    }

    /// Build a single config for a specific platform- or profile-specific configuration.
    fn from_specific_metadata(metadata: Option<&Value>) -> anyhow::Result<Self> {
        let Some(metadata) = metadata else {
            return Ok(Self::default());
        };
        let Value::Object(metadata) = metadata else {
            bail!("Bevy CLI config must be a table");
        };

        Ok(Self {
            target: extract_target(metadata)?,
            features: extract_features(metadata)?,
            default_features: extract_default_features(metadata)?,
            rustflags: extract_rustflags(metadata)?,
            wasm_opt: extract_use_wasm_opt(metadata)?,
        })
    }

    /// Merge another config into this one.
    ///
    /// The other config takes precedence,
    /// it's values overwrite the current values if one has to be chosen.
    pub fn overwrite(mut self, with: &Self) -> Self {
        self.target = with.target.clone().or(self.target);
        self.default_features = with.default_features.or(self.default_features);

        self.wasm_opt = with.wasm_opt.or(self.wasm_opt);

        // Features and Rustflags are additive
        self.features.extend(with.features.iter().cloned());
        self.rustflags.extend(with.rustflags.iter().cloned());

        self
    }
}

/// Try to extract the target platform from a metadata map for the CLI.
fn extract_target(cli_metadata: &Map<String, Value>) -> anyhow::Result<Option<String>> {
    let Some(target) = cli_metadata.get("target") else {
        return Ok(None);
    };

    match target {
        Value::String(target) => Ok(Some(target).cloned()),
        Value::Null => Ok(None),
        _ => bail!("target must be a string"),
    }
}

/// Try to extract the list of features from a metadata map for the CLI.
fn extract_features(cli_metadata: &Map<String, Value>) -> anyhow::Result<Vec<String>> {
    let Some(features) = cli_metadata.get("features") else {
        return Ok(Vec::new());
    };

    match features {
        Value::Array(features) => features
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(|str| str.to_string())
                    .ok_or_else(|| anyhow::anyhow!("each feature must be a string"))
            })
            .collect(),
        Value::Null => Ok(Vec::new()),
        _ => bail!("features must be an array"),
    }
}

/// Try to extract whether default-features are enabled from a metadata map for the CLI.
fn extract_default_features(cli_metadata: &Map<String, Value>) -> anyhow::Result<Option<bool>> {
    if let Some(default_features) = cli_metadata.get("default-features") {
        match default_features {
            Value::Bool(default_features) => Ok(Some(default_features).copied()),
            Value::Null => Ok(None),
            _ => bail!("default-features must be a boolean"),
        }
    } else if let Some(default_features) = cli_metadata.get("default_features") {
        warn!(
            "`default_features` has been renamed to `default-features` to align with Cargo's naming conventions."
        );
        match default_features {
            Value::Bool(default_features) => Ok(Some(default_features).copied()),
            Value::Null => Ok(None),
            _ => bail!("default_features must be a boolean"),
        }
    } else {
        return Ok(None);
    }
}

fn extract_rustflags(cli_metadata: &Map<String, Value>) -> anyhow::Result<Vec<String>> {
    let Some(rustflags) = cli_metadata.get("rustflags") else {
        return Ok(Vec::new());
    };

    match rustflags {
        Value::Array(rustflags) => rustflags
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(std::string::ToString::to_string)
                    .ok_or_else(|| anyhow::anyhow!("each rustflag must be a string"))
            })
            .collect(),
        Value::String(rustflag) => Ok(vec![rustflag.clone()]),
        Value::Null => Ok(Vec::new()),
        _ => bail!("rustflags must be an array or string"),
    }
}

fn extract_use_wasm_opt(cli_metadata: &Map<String, Value>) -> anyhow::Result<Option<bool>> {
    if let Some(use_wasm_opt) = cli_metadata.get("wasm-opt") {
        match use_wasm_opt {
            Value::Bool(use_wasm_opt) => Ok(Some(use_wasm_opt).copied()),
            Value::Null => Ok(None),
            _ => bail!("wasm-opt must be a boolean"),
        }
    } else {
        Ok(None)
    }
}

impl Display for CliConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let document = toml_edit::ser::to_document(self).map_err(|_| std::fmt::Error)?;
        write!(
            f,
            "{}",
            document
                .to_string()
                // Remove trailing newline
                .trim_end()
                .lines()
                // Align lines with the debug message
                .map(|line| format!("      {line}"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod merged_from_metadata {
        use serde_json::json;

        use super::*;

        #[test]
        fn should_return_merged_config_for_web_dev() -> anyhow::Result<()> {
            let metadata = json!({
                "rustflags": ["-C opt-level=2"],
                "features": ["base"],
                "dev": {
                    "features": ["dev"],
                },
                "web": {
                    "features": ["web"],
                    "default-features": false,
                    "dev": {
                        "features": ["web-dev"],
                        "rustflags": ["--cfg","getrandom_backend=\"wasm_js\""]
                    },
                }
            });

            assert_eq!(
                CliConfig::merged_from_metadata(Some(&metadata), true, false)?,
                CliConfig {
                    target: None,
                    features: vec![
                        "base".to_owned(),
                        "dev".to_owned(),
                        "web".to_owned(),
                        "web-dev".to_owned()
                    ],
                    default_features: Some(false),
                    rustflags: vec![
                        "-C opt-level=2".to_string(),
                        "--cfg".to_string(),
                        "getrandom_backend=\"wasm_js\"".to_string()
                    ],
                    wasm_opt: None
                }
            );
            Ok(())
        }

        #[test]
        fn should_return_merged_config_for_native_release() -> anyhow::Result<()> {
            let metadata = json!({
                "rustflags": ["-C opt-level=2"],
                "features": ["base"],
                "release": {
                    "features": ["release"],
                },
                "native": {
                    "features": ["native"],
                    "default-features": false,
                    "release": {
                        "features": ["native-release"],
                        "rustflags": ["-C debuginfo=1"]
                    }
                },
                "web": {
                    "features": ["web"],
                    "default-features": false,
                    "rustflags": ["--cfg","getrandom_backend=\"wasm_js\""]
                }
            });

            assert_eq!(
                CliConfig::merged_from_metadata(Some(&metadata), false, true)?,
                CliConfig {
                    target: None,
                    features: vec![
                        "base".to_owned(),
                        "release".to_owned(),
                        "native".to_owned(),
                        "native-release".to_owned()
                    ],
                    default_features: Some(false),
                    rustflags: vec!["-C opt-level=2".to_string(), "-C debuginfo=1".to_string()],
                    wasm_opt: None
                }
            );
            Ok(())
        }

        #[test]
        fn should_return_merged_config_for_native_dev() -> anyhow::Result<()> {
            let metadata = json!({
                "features": ["native-dev"],
                "dev": {
                    "features": [
                        "bevy/dynamic_linking",
                        "bevy/bevy_dev_tools",
                        "bevy/bevy_ui_debug"
                    ],
                    "default-features": true,
                },
                "web": {
                    "features": ["web"],
                    "default-features": false,
                    "dev": {
                        "features": ["web-dev"],
                    }
                }
            });

            assert_eq!(
                CliConfig::merged_from_metadata(Some(&metadata), false, false)?,
                CliConfig {
                    target: None,
                    features: vec![
                        "native-dev".to_owned(),
                        "bevy/dynamic_linking".to_owned(),
                        "bevy/bevy_dev_tools".to_owned(),
                        "bevy/bevy_ui_debug".to_owned()
                    ],
                    default_features: Some(true),
                    rustflags: Vec::new(),
                    wasm_opt: None
                }
            );
            Ok(())
        }

        #[test]
        fn should_not_require_any_config() -> anyhow::Result<()> {
            let metadata = json!({});

            assert_eq!(
                CliConfig::merged_from_metadata(Some(&metadata), true, false)?,
                CliConfig::default()
            );
            Ok(())
        }

        #[test]
        fn should_ignore_unrelated_configs() -> anyhow::Result<()> {
            let metadata = json!({
                "features": ["base"],
                "dev": {
                    "rustflags": ["-C opt-level=2"],
                    "features": ["dev"],
                    "default-features": true,
                },
                "web": {
                    "features": ["web"],
                    "default-features": false,
                    "rustflags": ["--cfg","getrandom_backend=\"wasm_js\""],
                    "dev": {
                        "rustflags": ["-C debuginfo=1"],
                        "features": ["web-dev"],
                    }
                }
            });

            assert_eq!(
                CliConfig::merged_from_metadata(Some(&metadata), false, true)?,
                CliConfig {
                    target: None,
                    features: vec!["base".to_owned(),],
                    default_features: None,
                    rustflags: Vec::new(),
                    wasm_opt: None
                }
            );
            Ok(())
        }
    }

    mod extract_target {
        use serde_json::Map;

        use super::*;

        #[test]
        fn should_return_none_if_no_target_specified() -> anyhow::Result<()> {
            let cli_metadata = Map::new();
            assert_eq!(extract_target(&cli_metadata)?, None);
            Ok(())
        }

        #[test]
        fn should_return_target_if_specified() -> anyhow::Result<()> {
            let mut cli_metadata = Map::new();
            cli_metadata.insert("target".to_owned(), "wasm32v1-none".into());
            assert_eq!(
                extract_target(&cli_metadata)?,
                Some("wasm32v1-none".to_string())
            );
            Ok(())
        }

        #[test]
        fn should_return_error_if_target_is_not_a_string() {
            let mut cli_metadata = Map::new();
            cli_metadata.insert("target".to_string(), 32.into());
            assert!(extract_target(&cli_metadata).is_err());
        }
    }

    mod extract_features {
        use serde_json::Map;

        use super::*;

        #[test]
        fn should_return_empty_vec_if_no_features_specified() -> anyhow::Result<()> {
            let cli_metadata = Map::new();
            assert_eq!(extract_features(&cli_metadata)?, Vec::<String>::new());
            Ok(())
        }

        #[test]
        fn should_return_features_if_listed() -> anyhow::Result<()> {
            let mut cli_metadata = Map::new();
            cli_metadata.insert("features".to_owned(), vec!["dev", "web"].into());
            assert_eq!(
                extract_features(&cli_metadata)?,
                vec!["dev".to_owned(), "web".to_owned()]
            );
            Ok(())
        }

        #[test]
        fn should_return_error_if_one_feature_is_not_a_string() {
            let mut cli_metadata = Map::new();
            cli_metadata.insert(
                "features".to_string(),
                vec![
                    Value::String("dev".to_owned()),
                    Value::Bool(false),
                    Value::Null,
                ]
                .into(),
            );
            assert!(extract_features(&cli_metadata).is_err());
        }
    }
}
