use clap::Args;

use crate::config::CliConfig;

/// Common arguments shared by most of the commands.
#[derive(Debug, Clone, Args)]
pub struct CommonArgs {
    #[cfg(feature = "unstable")]
    #[clap(flatten)]
    pub unstable: UnstableArgs,
}

impl CommonArgs {
    /// Headers to add to the local web server.
    #[cfg(not(feature = "unstable"))]
    pub fn web_headers(&self) -> Vec<String> {
        Vec::new()
    }

    /// Headers to add to the local web server.
    #[cfg(feature = "unstable")]
    pub fn web_headers(&self) -> Vec<String> {
        self.unstable.web_headers()
    }

    /// Apply the settings configured in `Cargo.toml`.
    #[cfg(not(feature = "unstable"))]
    pub(crate) fn apply_config(&mut self, _config: &CliConfig) {}

    /// Apply the settings configured in `Cargo.toml`.
    #[cfg(feature = "unstable")]
    pub(crate) fn apply_config(&mut self, config: &CliConfig) {
        self.unstable.apply_config(config);
    }
}

/// Unstable arguments that are experimental.
#[cfg(feature = "unstable")]
#[derive(Debug, Clone, Args)]
pub struct UnstableArgs {
    /// Enable experimental and unstable features.
    ///
    /// Please note that these features...
    ///
    /// - might be removed or drastically changed in future releases
    /// - might require additional setup or workarounds
    #[arg(short = 'U', long = "unstable", value_name = "FEATURE")]
    pub unstable_features: Vec<String>,
}

#[cfg(feature = "unstable")]
impl UnstableArgs {
    /// Whether the user has enabled Wasm multi-threading features.
    pub fn web_multi_threading(&self) -> bool {
        self.unstable_features
            .contains(&"web-multi-threading".to_string())
    }

    /// Headers to add to the local web server.
    pub fn web_headers(&self) -> Vec<String> {
        if self.web_multi_threading() {
            // Make the document cross-origin isolated,
            // which is required for Wasm multi-threading
            // See also https://developer.mozilla.org/en-US/docs/Web/API/Window/crossOriginIsolated
            vec![
                "cross-origin-opener-policy=same-origin".to_owned(),
                "cross-origin-embedder-policy=require-corp".to_owned(),
            ]
        } else {
            Vec::new()
        }
    }

    /// Apply the settings configured in `Cargo.toml`.
    pub(crate) fn apply_config(&mut self, config: &CliConfig) {
        if !self.web_multi_threading()
            && config
                .web_multi_threading()
                .is_some_and(|multi_threading| multi_threading)
        {
            self.unstable_features
                .push("web-multi-threading".to_string());
        }
    }
}
