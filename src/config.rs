//! Configuration used by the `bevy_cli`, defined in `Cargo.toml` under `package.metadata.bevy_cli`.
use std::fmt::Display;

use mergeme::Merge;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Configuration for the `bevy_cli`.
/// 
/// [`PartialCliConfig`]s are intended to be deserialized from `[package.metadata.bevy_cli]` and
/// merged into this struct.
#[derive(Merge, PartialEq, Serialize, Debug)]
#[partial(
    PartialCliConfig,
    derive(Deserialize),
    serde(rename_all = "kebab-case")
)]
#[serde(rename_all = "kebab-case")]
pub struct CliConfig {
    /// The platform to target with the build.
    target: Option<String>,

    /// Additional features that should be enabled.
    ///
    /// Features are additive when merged.
    #[strategy(merge)]
    features: Vec<String>,

    /// Whether to use default features.
    default_features: bool,

    /// Additional flags for `rustc`
    ///
    /// Rust flags are additive when merged.
    #[strategy(merge)]
    rustflags: Vec<String>,

    /// Use `wasm-opt` to optimize wasm binaries.
    wasm_opt: Option<bool>,
}

impl CliConfig {
    pub fn from_metadata(
        package_metadata: &Value,
        is_web: bool,
        is_release: bool,
    ) -> anyhow::Result<Self> {
        let profile = if is_release { "release" } else { "dev" };
        let target = if is_web { "web" } else { "native" };

        // The base configuration that all partial config gets merged into.
        let mut config = Self::default();

        // Return the `[package.metadata.bevy_cli]` table for the current package, if it exists.
        let cli_metadata = package_metadata.get("bevy_cli");
        let profile_metadata = cli_metadata.and_then(|m| m.get(profile));
        let target_metadata = cli_metadata.and_then(|m| m.get(target));
        let target_profile_metadata = target_metadata.and_then(|m| m.get(profile));

        // Deserialise each `Value` into a `PartialCliConfig`, then merge it into the default. See
        // <https://thebevyflock.github.io/bevy_cli/cli/configuration.html#configuration-merging>
        // for more info.
        for metadata in [
            cli_metadata,
            profile_metadata,
            target_metadata,
            target_profile_metadata,
        ] {
            let Some(metadata) = metadata else {
                continue;
            };

            // Deserialize metadata into a `PartialCliConfig`, then merge it with the base.
            config.merge_in_place(serde_json::from_value(metadata.clone())?);
        }

        Ok(config)
    }

    /// Returns true if this config is equivalent to that returned by [`CliConfig::default()`].
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }

    /// The platform to target with the build.
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    /// The features enabled in the config.
    pub fn features(&self) -> &[String] {
        &self.features
    }

    /// Whether to enable default features.
    pub fn default_features(&self) -> bool {
        self.default_features
    }

    /// The rustflags enabled in the config.
    /// 
    /// This is automatically formatted so that it may be passed to the `RUSTFLAGS` environmental
    /// variable.
    pub fn rustflags(&self) -> Option<String> {
        if self.rustflags.is_empty() {
            return None;
        }

        Some(self.rustflags.join(" "))
    }

    /// Whether to use `wasm-opt`.
    #[cfg(feature = "web")]
    pub fn wasm_opt(&self) -> Option<bool> {
        self.wasm_opt
    }
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            target: None,
            features: Vec::new(),
            default_features: true,
            rustflags: Vec::new(),
            wasm_opt: None,
        }
    }
}

impl Display for CliConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Serialize this struct into TOML.
        let document = toml_edit::ser::to_string(self).map_err(|_| std::fmt::Error)?;

        write!(
            f,
            "{}",
            document
                // Remove trailing newline.
                .trim_end()
                .lines()
                // Align lines with the debug message.
                .map(|line| format!("      {line}"))
                // Join all lines together with a newline in between.
                .reduce(|acc, line| acc + "\n" + line.as_ref())
                .unwrap_or(String::new())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_metadata {
        use serde_json::json;

        use super::*;

        #[test]
        fn should_return_merged_config_for_web_dev() -> anyhow::Result<()> {
            let metadata = json!({
                "bevy_cli": {
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
                            "rustflags": ["--cfg","getrandom_backend=\"wasm_js\""],
                        },
                    },
                },
            });

            assert_eq!(
                CliConfig::from_metadata(&metadata, true, false)?,
                CliConfig {
                    target: None,
                    features: vec![
                        "base".to_owned(),
                        "dev".to_owned(),
                        "web".to_owned(),
                        "web-dev".to_owned()
                    ],
                    default_features: false,
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
                "bevy_cli": {
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
                            "rustflags": ["-C debuginfo=1"],
                        },
                    },
                    "web": {
                        "features": ["web"],
                        "default-features": false,
                        "rustflags": ["--cfg","getrandom_backend=\"wasm_js\""],
                    },
                },
            });

            assert_eq!(
                CliConfig::from_metadata(&metadata, false, true)?,
                CliConfig {
                    target: None,
                    features: vec![
                        "base".to_owned(),
                        "release".to_owned(),
                        "native".to_owned(),
                        "native-release".to_owned()
                    ],
                    default_features: false,
                    rustflags: vec!["-C opt-level=2".to_string(), "-C debuginfo=1".to_string()],
                    wasm_opt: None
                }
            );

            Ok(())
        }

        #[test]
        fn should_return_merged_config_for_native_dev() -> anyhow::Result<()> {
            let metadata = json!({
                "bevy_cli": {
                    "features": ["native-dev"],
                    "dev": {
                        "features": [
                            "bevy/dynamic_linking",
                            "bevy/bevy_dev_tools",
                            "bevy/bevy_ui_debug",
                        ],
                        "default-features": true,
                    },
                    "web": {
                        "features": ["web"],
                        "default-features": false,
                        "dev": {
                            "features": ["web-dev"],
                        },
                    },
                },
            });

            assert_eq!(
                CliConfig::from_metadata(&metadata, false, false)?,
                CliConfig {
                    target: None,
                    features: vec![
                        "native-dev".to_owned(),
                        "bevy/dynamic_linking".to_owned(),
                        "bevy/bevy_dev_tools".to_owned(),
                        "bevy/bevy_ui_debug".to_owned()
                    ],
                    default_features: true,
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
                CliConfig::from_metadata(&metadata, true, false)?,
                CliConfig::default()
            );

            Ok(())
        }

        #[test]
        fn should_ignore_unrelated_configs() -> anyhow::Result<()> {
            let metadata = json!({
                "bevy_cli": {
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
                        },
                    },
                },
            });

            assert_eq!(
                CliConfig::from_metadata(&metadata, false, true)?,
                CliConfig {
                    target: None,
                    features: vec!["base".to_owned(),],
                    default_features: true,
                    rustflags: Vec::new(),
                    wasm_opt: None
                }
            );

            Ok(())
        }
    }
}
