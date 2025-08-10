use clap::{Args, ValueEnum};

#[derive(Debug, Default, Clone, Args)]
pub struct UnstableWebArgs {
    /// Enable unstable web features.
    #[arg(short = 'U', long = "unstable", name = "WEB FEATURE", value_enum)]
    pub unstable_features: Vec<UnstableWebFeature>,
}

impl UnstableWebArgs {
    /// Whether the user has enabled multi-threading features for web.
    pub fn web_multi_threading(&self) -> bool {
        self.unstable_features
            .contains(&UnstableWebFeature::MultiThreading)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum UnstableWebFeature {
    /// Enable building and running multi-threaded Wasm apps.
    ///
    /// Requires nightly Rust.
    MultiThreading,
}
