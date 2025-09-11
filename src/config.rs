//! Configuration used by the `bevy_cli`, defined in `Cargo.toml` under `package.metadata.bevy_cli`.
use std::fmt::Display;

use anyhow::{Context, bail};
use cargo_metadata::{Metadata, Package};
use serde::Serialize;
use serde_json::{Map, Value};
use tracing::warn;

use crate::external_cli::external_cli_args::ExternalCliArgs;

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
    /// Additional HTTP headers to send with requests.
    headers: Vec<String>,
    /// Additional flags for `rustc`
    rustflags: Vec<String>,
    /// Use `wasm-opt` to optimize wasm binaries.
    wasm_opt: Option<ExternalCliArgs>,
    /// EXPERIMENTAL: Enable building and running apps that use Wasm multi-threading features.
    web_multi_threading: Option<bool>,
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
            web_multi_threading,
            headers,
        } = self;

        target.is_none()
            && features.is_empty()
            && default_features.is_none()
            && rustflags.is_empty()
            && wasm_opt.is_none()
            && web_multi_threading.is_none()
            && headers.is_empty()
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

    /// Whether to enable Wasm multi-threading functionality.
    #[cfg(feature = "unstable")]
    pub fn web_multi_threading(&self) -> Option<bool> {
        self.web_multi_threading
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

    /// The `wasm-opt` configuration.
    #[cfg(feature = "web")]
    pub fn wasm_opt(&self, is_release: bool) -> ExternalCliArgs {
        self.wasm_opt.clone().unwrap_or({
            // Enable by default for release builds
            if is_release {
                ExternalCliArgs::Enabled(true)
            } else {
                ExternalCliArgs::Enabled(false)
            }
        })
    }

    /// Additional HTTP headers to send with requests.
    #[cfg(feature = "web")]
    pub fn headers(&self) -> Vec<String> {
        self.headers.clone()
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

        let unstable_config = extract_unstable_config(metadata)?;

        Ok(Self {
            target: extract_target(metadata)?,
            features: extract_features(metadata)?,
            default_features: extract_default_features(metadata)?,
            rustflags: extract_rustflags(metadata)?,
            wasm_opt: extract_wasm_opt(metadata)?,
            web_multi_threading: extract_web_multi_threading(unstable_config)?,
            headers: extract_headers(metadata)?,
        })
    }

    /// Merge another config into this one.
    ///
    /// The other config takes precedence,
    /// it's values overwrite the current values if one has to be chosen.
    pub fn overwrite(self, with: &Self) -> Self {
        Self {
            target: with.target.clone().or(self.target),
            default_features: with.default_features.or(self.default_features),
            wasm_opt: with.wasm_opt.clone().or(self.wasm_opt),
            // Features, rustflags and headers are additive
            features: [self.features, with.features.clone()].concat(),
            rustflags: [self.rustflags, with.rustflags.clone()].concat(),
            headers: [self.headers, with.headers.clone()].concat(),
            web_multi_threading: with.web_multi_threading.or(self.web_multi_threading),
        }
    }

    /// Append rustflags from a resolved cargo config to the [`CliConfig`] rustflags.
    pub fn append_cargo_config_rustflags(
        &mut self,
        target: Option<String>,
        config: &cargo_config2::Config,
    ) -> anyhow::Result<()> {
        // Use the explicitly provided target, or fall back to the system's host triple.
        let target = {
            match target {
                Some(target_args) => target_args,
                None => config.host_triple()?.to_owned(),
            }
        };

        // Read the rustflags from set environment variables and merged Cargo config's for the
        // given target and append them to the rustflags from the Cli config.
        if let Some(cargo_config_rustflags) = config.rustflags(target)? {
            self.rustflags
                .extend(cargo_config_rustflags.flags.iter().cloned());
        }

        Ok(())
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
        Ok(None)
    }
}

/// Try to extract additional web headers from the metadata map for the CLI.
fn extract_headers(cli_metadata: &Map<String, Value>) -> anyhow::Result<Vec<String>> {
    const KEY: &str = "headers";

    let Some(features) = cli_metadata.get(KEY) else {
        return Ok(Vec::new());
    };

    match features {
        Value::Array(features) => features
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(|str| str.to_string())
                    .ok_or_else(|| anyhow::anyhow!("each header must be a string"))
            })
            .collect(),
        Value::Null => Ok(Vec::new()),
        _ => bail!("{KEY} must be an array"),
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

fn extract_wasm_opt(cli_metadata: &Map<String, Value>) -> anyhow::Result<Option<ExternalCliArgs>> {
    if let Some(wasm_opt) = cli_metadata.get("wasm-opt") {
        match wasm_opt {
            Value::Bool(enabled) => Ok(Some(ExternalCliArgs::Enabled(*enabled))),
            Value::Array(arr) => {
                let args = arr
                    .iter()
                    .map(|value| {
                        value
                            .as_str()
                            .map(std::string::ToString::to_string)
                            .ok_or_else(|| {
                                anyhow::anyhow!("each wasm-opt argument must be a string")
                            })
                    })
                    .collect::<Result<Vec<String>, _>>()?;
                Ok(Some(ExternalCliArgs::Args(args)))
            }
            Value::Null => Ok(None),
            _ => bail!("wasm-opt must be a boolean or an array of arguments to pass to wasm-opt"),
        }
    } else {
        Ok(None)
    }
}

/// Try to extract the map containing unstable CLI features.
fn extract_unstable_config(
    cli_metadata: &Map<String, Value>,
) -> anyhow::Result<Option<&Map<String, Value>>> {
    const KEY: &str = "unstable";

    if let Some(unstable) = cli_metadata.get(KEY) {
        match unstable {
            Value::Object(unstable) => Ok(Some(unstable)),
            _ => bail!("{KEY} must be a map"),
        }
    } else {
        Ok(None)
    }
}

/// Try to extract whether multi-threading features for the web are enabled from a metadata map for
/// the CLI.
fn extract_web_multi_threading(
    unstable_config: Option<&Map<String, Value>>,
) -> anyhow::Result<Option<bool>> {
    const KEY: &str = "web-multi-threading";

    let Some(unstable_config) = unstable_config else {
        return Ok(None);
    };

    if let Some(web_multi_threading) = unstable_config.get(KEY) {
        match web_multi_threading {
            Value::Bool(web_multi_threading) => Ok(Some(web_multi_threading).copied()),
            Value::Null => Ok(None),
            _ => bail!("{KEY} must be a boolean"),
        }
    } else {
        Ok(None)
    }
}

impl Display for CliConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let document = toml::to_string_pretty(self).map_err(|_| std::fmt::Error)?;
        write!(
            f,
            "{}",
            document
                .to_string()
                // Remove trailing newline
                .trim_end()
                .lines()
                // Align lines with the debug message
                .map(|line| format!("       {line}"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
