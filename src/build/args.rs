use clap::{ArgAction, Args, Subcommand};

use crate::external_cli::arg_builder::ArgBuilder;

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// The subcommands available for the build command.
    #[clap(subcommand)]
    pub subcommand: Option<BuildSubcommands>,

    /// Package to build (see `cargo help pkgid`).
    #[clap(short = 'p', long = "package", value_name = "SPEC")]
    pub package: Option<String>,

    /// Build all packages in the workspace.
    #[clap(long = "workspace", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_workspace: bool,

    ///  Exclude packages from the build.
    #[clap(long = "exclude", value_name = "SPEC")]
    pub exclude: Option<String>,

    /// Build only this package's library.
    #[clap(long = "lib", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_lib: bool,

    /// Require Cargo.lock is up to date.
    #[clap(long = "locked", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_locked: bool,

    /// Build only the specified binary.
    #[clap(long = "bin", value_name = "NAME")]
    pub bin: Option<String>,

    /// Build all binaries.
    #[clap(long = "bins", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_bins: bool,

    /// Build only the specified example.
    #[clap(long = "example", value_name = "NAME")]
    pub example: Option<String>,

    /// Build all examples.
    #[clap(long = "examples", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_examples: bool,

    /// Build only the specified test target.
    #[clap(long = "test", value_name = "NAME")]
    pub test: Option<String>,

    /// Build all tests.
    #[clap(long = "tests", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_tests: bool,

    /// Build only the specified bench target.
    #[clap(long = "bench", value_name = "NAME")]
    pub bench: Option<String>,

    /// Build all benches.
    #[clap(long = "benches", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_benches: bool,

    /// Build all benches.
    #[clap(long = "all-targets", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_all_targets: bool,

    /// Build artifacts in release mode, with optimizations.
    #[clap(short = 'r', long = "release", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_release: bool,

    /// Build for the target triple.
    #[clap(long = "target", value_name = "TRIPLE")]
    pub target: Option<String>,

    /// Directory for all generated artifacts.
    #[clap(long = "target-dir", value_name = "DIRECTORY")]
    pub target_dir: Option<String>,

    /// Path to Cargo.toml.
    #[clap(long = "manifest-path", value_name = "PATH")]
    pub manifest_path: Option<String>,
}

impl BuildArgs {
    /// Determine if the app is being built for the web.
    pub(crate) fn is_web(&self) -> bool {
        matches!(self.subcommand, Some(BuildSubcommands::Web))
    }

    /// Generate arguments for `cargo`.
    pub(crate) fn cargo_args(&self) -> ArgBuilder {
        // --web takes precedence over --target <TRIPLE>
        let target = if self.is_web() {
            Some("wasm32-unknown-unknown".to_string())
        } else {
            self.target.clone()
        };

        ArgBuilder::new()
            .add_opt_value("--package", &self.package)
            .add_flag_if("--workspace", self.is_workspace)
            .add_opt_value("--exclude", &self.exclude)
            .add_flag_if("--lib", self.is_lib)
            .add_flag_if("--locked", self.is_locked)
            .add_opt_value("--bin", &self.bin)
            .add_flag_if("--bins", self.is_bins)
            .add_opt_value("--example", &self.example)
            .add_flag_if("--examples", self.is_examples)
            .add_opt_value("--test", &self.test)
            .add_flag_if("--tests", self.is_tests)
            .add_opt_value("--bench", &self.bench)
            .add_flag_if("--benches", self.is_benches)
            .add_flag_if("--all-targets", self.is_all_targets)
            .add_flag_if("--release", self.is_release)
            .add_opt_value("--target", &target)
            .add_opt_value("--target-dir", &self.target_dir)
            .add_opt_value("--manifest-path", &self.manifest_path)
    }
}

#[derive(Debug, Subcommand)]
pub enum BuildSubcommands {
    /// Build your app for the browser.
    Web,
}
