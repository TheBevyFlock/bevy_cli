use clap::Args;

#[cfg(feature = "unstable")]
use crate::config::CliConfig;

#[derive(Debug, Clone, Args)]
pub struct CommonArgs {
    #[cfg(feature = "unstable")]
    #[clap(flatten)]
    pub unstable: UnstableArgs,
}

impl CommonArgs {
    #[cfg(not(feature = "unstable"))]
    pub fn web_headers(&self) -> Vec<String> {
        Vec::new()
    }

    #[cfg(feature = "unstable")]
    pub fn web_headers(&self) -> Vec<String> {
        self.unstable.web_headers()
    }

    pub(crate) fn apply_config(&mut self, config: &CliConfig) {
        #[cfg(feature = "unstable")]
        self.unstable.apply_config(config);
    }
}

#[cfg(feature = "unstable")]
#[derive(Debug, Clone, Args)]
pub struct UnstableArgs {
    /// Enable experimental and unstable features.
    ///
    /// Please note that these features...
    ///
    /// - might be removed or drastically changed in future releases
    /// - might require additional setup or workarounds
    #[arg(short = 'U', long = "unstable", value_name = "FEATURE", global = true)]
    pub unstable_features: Vec<String>,
}

#[cfg(feature = "unstable")]
impl UnstableArgs {
    /// Whether the user has enabled Wasm multi-threading features.
    pub fn web_multi_threading(&self) -> bool {
        self.unstable_features
            .contains(&"web-multi-threading".to_string())
    }

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
