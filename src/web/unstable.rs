use clap::{Args, ValueEnum};

#[derive(Debug, Default, Clone, Args)]
pub struct UnstableWebArgs {
    /// Enable unstable web features.
    #[arg(short = 'U', long = "unstable", value_enum)]
    pub unstable_features: Vec<UnstableWebFeature>,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum UnstableWebFeature {
    /// Enable building and running multi-threaded Wasm apps.
    MultiThreading,
}
