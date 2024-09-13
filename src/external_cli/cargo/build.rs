use std::process::Command;

use clap::{ArgAction, Args};

use crate::external_cli::arg_builder::ArgBuilder;

use super::PROGRAM;

/// Create a command to run `cargo build`.
pub(crate) fn command() -> Command {
    let mut command = Command::new(PROGRAM);
    command.arg("build");
    command
}

#[derive(Debug, Args)]
pub struct CargoBuildArgs {
    #[clap(flatten)]
    pub package_args: CargoPackageBuildArgs,
    #[clap(flatten)]
    pub target_args: CargoTargetBuildArgs,
    #[clap(flatten)]
    pub feature_args: CargoFeatureBuildArgs,
    #[clap(flatten)]
    pub compilation_args: CargoCompilationBuildArgs,
    #[clap(flatten)]
    pub manifest_args: CargoManifestBuildArgs,
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Package Selection")]
pub struct CargoPackageBuildArgs {
    /// Package to build (see `cargo help pkgid`)
    #[clap(short = 'p', long = "package", value_name = "SPEC")]
    pub package: Option<String>,

    /// Build all packages in the workspace
    #[clap(long = "workspace", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_workspace: bool,

    /// Exclude packages from the build
    #[clap(long = "exclude", value_name = "SPEC")]
    pub exclude: Option<String>,
}

impl CargoPackageBuildArgs {
    pub(crate) fn args_builder(&self) -> ArgBuilder {
        ArgBuilder::new()
            .add_opt_value("--package", &self.package)
            .add_flag_if("--workspace", self.is_workspace)
            .add_opt_value("--exclude", &self.exclude)
    }
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Target Selection")]
pub struct CargoTargetBuildArgs {
    /// Build only this package's library
    #[clap(long = "lib", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_lib: bool,

    /// Build all binaries.
    #[clap(long = "bins", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_bins: bool,

    /// Build only the specified binary.
    #[clap(long = "bin", value_name = "NAME")]
    pub bin: Option<String>,

    /// Build all examples.
    #[clap(long = "examples", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_examples: bool,

    /// Build only the specified example.
    #[clap(long = "example", value_name = "NAME")]
    pub example: Option<String>,

    /// Build all tests.
    #[clap(long = "tests", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_tests: bool,

    /// Build only the specified test target.
    #[clap(long = "test", value_name = "NAME")]
    pub test: Option<String>,

    /// Build all benches.
    #[clap(long = "benches", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_benches: bool,

    /// Build only the specified bench target.
    #[clap(long = "bench", value_name = "NAME")]
    pub bench: Option<String>,

    /// Build all benches.
    #[clap(long = "all-targets", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_all_targets: bool,
}

impl CargoTargetBuildArgs {
    pub(crate) fn args_builder(&self) -> ArgBuilder {
        ArgBuilder::new()
            .add_flag_if("--lib", self.is_lib)
            .add_flag_if("--bins", self.is_bins)
            .add_opt_value("--bin", &self.bin)
            .add_flag_if("--examples", self.is_examples)
            .add_opt_value("--example", &self.example)
            .add_flag_if("--tests", self.is_tests)
            .add_opt_value("--test", &self.test)
            .add_flag_if("--benches", self.is_benches)
            .add_opt_value("--bench", &self.bench)
            .add_flag_if("--all-targets", self.is_all_targets)
    }
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Feature Selection")]
pub struct CargoFeatureBuildArgs {
    /// Space or comma separated list of features to activate
    #[clap(short = 'F', long = "features", value_name = "FEATURES")]
    pub features: Vec<String>,

    /// Activate all available features
    #[clap(long = "all-features", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_all_features: bool,

    /// Do not activate the `default` feature
    #[clap(long = "no-default-features", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_no_default_features: bool,
}

impl CargoFeatureBuildArgs {
    pub(crate) fn args_builder(&self) -> ArgBuilder {
        ArgBuilder::new()
            .add_value_list("--features", self.features.clone())
            .add_flag_if("--all-features", self.is_all_features)
            .add_flag_if("--no-default-features", self.is_no_default_features)
    }
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Compilation Options")]
pub struct CargoCompilationBuildArgs {
    /// Build artifacts in release mode, with optimizations.
    #[clap(short = 'r', long = "release", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_release: bool,

    /// Do not activate the `default` feature
    #[clap(long = "no-default-features", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_no_default_features: bool,

    /// Build artifacts with the specified profile
    #[clap(long = "profile", value_name = "PROFILE-NAME")]
    pub profile: Option<String>,

    /// Number of parallel jobs, defaults to # of CPUs.
    #[clap(short = 'j', long = "jobs", value_name = "N")]
    pub jobs: Option<u32>,

    /// Do not abort the build as soon as there is an error
    #[clap(long = "keep-going", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_keep_going: bool,

    /// Build for the target triple.
    #[clap(long = "target", value_name = "TRIPLE")]
    pub target: Option<String>,

    /// Directory for all generated artifacts.
    #[clap(long = "target-dir", value_name = "DIRECTORY")]
    pub target_dir: Option<String>,
}

impl CargoCompilationBuildArgs {
    pub(crate) fn args_builder(&self, is_web: bool) -> ArgBuilder {
        // web takes precedence over --target <TRIPLE>
        let target = if is_web {
            Some("wasm32-unknown-unknown".to_string())
        } else {
            self.target.clone()
        };

        ArgBuilder::new()
            .add_flag_if("--release", self.is_release)
            .add_flag_if("--no-default-features", self.is_no_default_features)
            .add_opt_value("--profile", &self.profile)
            .add_opt_value("--jobs", &self.jobs.map(|jobs| jobs.to_string()))
            .add_flag_if("--keep-going", self.is_keep_going)
            .add_opt_value("--target", &target)
            .add_opt_value("--target-dir", &self.target_dir)
    }
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Manifest Options")]
pub struct CargoManifestBuildArgs {
    /// Path to Cargo.toml
    #[clap(long = "manifest-path", value_name = "PATH")]
    pub manifest_path: Option<String>,

    /// Ignore `rust-version` specification in packages
    #[clap(long = "ignore-rust-version", action = ArgAction::SetTrue, default_value_t = false)]
    pub ignore_rust_version: bool,

    /// Assert that `Cargo.lock` will remain unchanged
    #[clap(long = "locked", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_locked: bool,

    /// Run without accessing the network
    #[clap(long = "offline", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_offline: bool,

    /// Equivalent to specifying both --locked and --offline
    #[clap(long = "frozen", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_frozen: bool,
}

impl CargoManifestBuildArgs {
    pub(crate) fn args_builder(&self) -> ArgBuilder {
        ArgBuilder::new()
            .add_opt_value("--manifest-path", &self.manifest_path)
            .add_flag_if("--ignore-rust-version", self.ignore_rust_version)
            .add_flag_if("--locked", self.is_locked)
            .add_flag_if("--offline", self.is_offline)
            .add_flag_if("--frozen", self.is_frozen)
    }
}
