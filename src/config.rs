use anyhow::bail;
use serde_json::{Map, Value};

use crate::external_cli::cargo::metadata::{Metadata, Package};

#[derive(Default, Debug, Clone)]
pub struct CliConfig {
    /// Additional features that should be enabled.
    features: Vec<String>,
    /// Whether to use default features.
    default_features: Option<bool>,
}

impl CliConfig {
    /// Whether to enable default features.
    ///
    /// Defaults to `true` if not configured otherwise.
    pub fn default_features(&self) -> bool {
        self.default_features.unwrap_or(true)
    }

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

        let Some(cli_metadata) = package_metadata.get("bevy_cli") else {
            return Ok(Self::default());
        };

        let Value::Object(cli_metadata) = cli_metadata else {
            bail!("Bevy CLI config must be a table");
        };

        Ok(Self {
            features: extract_features(cli_metadata)?,
            default_features: None,
        })
    }

    /// Merge another config into this one.
    ///
    /// The other config takes precedence,
    /// it's values overwrite the current values if one has to be chosen.
    pub fn overwrite(&mut self, with: &Self) -> &mut Self {
        self.default_features = with.default_features.or(self.default_features);
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
        _ => bail!("package.metadata.bevy_cli.features must be an array"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            cli_metadata.insert("features".to_string(), vec!["dev", "web"].into());
            assert_eq!(
                extract_features(&cli_metadata)?,
                vec!["dev".to_string(), "web".to_string()]
            );
            Ok(())
        }
    }
}
