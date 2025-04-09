use anyhow::{Context, bail};
use serde_json::{Map, Value};

use crate::external_cli::cargo::metadata::{Metadata, Package};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CliConfig {
    /// Additional features that should be enabled.
    features: Vec<String>,
    /// Whether to use default features.
    default_features: Option<bool>,
    /// Additional flags for `rustc`
    rustflags: Option<Vec<String>>,
}

impl CliConfig {
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
        self.rustflags.as_ref().map(|flags| flags.join(" "))
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
            features: extract_features(metadata)?,
            default_features: extract_default_features(metadata)?,
            rustflags: extract_rustflags(metadata),
        })
    }

    /// Merge another config into this one.
    ///
    /// The other config takes precedence,
    /// it's values overwrite the current values if one has to be chosen.
    pub fn overwrite(mut self, with: &Self) -> Self {
        self.default_features = with.default_features.or(self.default_features);
        self.rustflags = with.rustflags.clone().or(self.rustflags);

        // Features are additive
        self.features.extend(with.features.iter().cloned());

        self
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

/// Try to extract whether default_features are enabled from a metadata map for the CLI.
fn extract_default_features(cli_metadata: &Map<String, Value>) -> anyhow::Result<Option<bool>> {
    let Some(default_features) = cli_metadata.get("default_features") else {
        return Ok(None);
    };

    match default_features {
        Value::Bool(default_features) => Ok(Some(default_features).copied()),
        Value::Null => Ok(None),
        _ => bail!("default_features must be an array"),
    }
}

fn extract_rustflags(cli_metadata: &Map<String, Value>) -> Option<Vec<String>> {
    let rustflags = cli_metadata.get("rustflags")?;
    match rustflags {
        Value::Array(features) => features
            .iter()
            .map(|value| value.as_str().map(std::string::ToString::to_string))
            .collect(),
        _ => None,
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
                "features": ["base"],
                "dev": {
                    "features": ["dev"],
                },
                "web": {
                    "features": ["web"],
                    "default_features": false,
                    "dev": {
                        "features": ["web-dev"],
                        "rustflags": ["--cfg","getrandom_backend=\"wasm_js\""]
                    },
                }
            });

            assert_eq!(
                CliConfig::merged_from_metadata(Some(&metadata), true, false)?,
                CliConfig {
                    features: vec![
                        "base".to_owned(),
                        "dev".to_owned(),
                        "web".to_owned(),
                        "web-dev".to_owned()
                    ],
                    default_features: Some(false),
                    rustflags: Some(vec![
                        "--cfg".to_string(),
                        "getrandom_backend=\"wasm_js\"".to_string()
                    ])
                }
            );
            Ok(())
        }

        #[test]
        fn should_return_merged_config_for_native_release() -> anyhow::Result<()> {
            let metadata = json!({
                "features": ["base"],
                "release": {
                    "features": ["release"],
                },
                "native": {
                    "features": ["native"],
                    "default_features": false,
                    "release": {
                        "features": ["native-release"],
                    }
                }
            });

            assert_eq!(
                CliConfig::merged_from_metadata(Some(&metadata), false, true)?,
                CliConfig {
                    features: vec![
                        "base".to_owned(),
                        "release".to_owned(),
                        "native".to_owned(),
                        "native-release".to_owned()
                    ],
                    default_features: Some(false),
                    rustflags: None
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
                    "features": ["dev"],
                    "default_features": true,
                },
                "web": {
                    "features": ["web"],
                    "default_features": false,
                    "dev": {
                        "features": ["web-dev"],
                    }
                }
            });

            assert_eq!(
                CliConfig::merged_from_metadata(Some(&metadata), false, true)?,
                CliConfig {
                    features: vec!["base".to_owned(),],
                    default_features: None,
                    rustflags: None
                }
            );
            Ok(())
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
