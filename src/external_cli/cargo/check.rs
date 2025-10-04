use clap::{ArgAction, Args};

use super::{CargoCommonArgs, CargoCompilationArgs, CargoFeatureArgs, CargoManifestArgs};
use crate::external_cli::arg_builder::ArgBuilder;

#[derive(Debug, Args)]
pub struct CargoCheckArgs {
    #[clap(flatten)]
    pub common_args: CargoCommonArgs,
    #[clap(flatten)]
    pub package_args: CargoPackageCheckArgs,
    #[clap(flatten)]
    pub target_args: CargoTargetCheckArgs,
    #[clap(flatten)]
    pub feature_args: CargoFeatureArgs,
    #[clap(flatten)]
    pub compilation_args: CargoCompilationArgs,
    #[clap(flatten)]
    pub manifest_args: CargoManifestArgs,
}

impl CargoCheckArgs {
    pub(crate) fn args_builder(&self, is_web: bool) -> ArgBuilder {
        ArgBuilder::new()
            .append(self.common_args.args_builder())
            .append(self.package_args.args_builder())
            .append(self.target_args.args_builder())
            .append(self.feature_args.args_builder())
            .append(self.compilation_args.args_builder(is_web))
            .append(self.manifest_args.args_builder())
    }
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Package Selection")]
pub struct CargoPackageCheckArgs {
    /// Package to check (see `cargo help pkgid`)
    #[clap(short = 'p', long = "package", value_name = "SPEC")]
    pub package: Option<String>,

    /// Check all packages in the workspace
    #[clap(long = "workspace", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_workspace: bool,

    /// Exclude packages from the check
    #[clap(long = "exclude", value_name = "SPEC")]
    pub exclude: Option<String>,
}

impl CargoPackageCheckArgs {
    pub(crate) fn args_builder(&self) -> ArgBuilder {
        ArgBuilder::new()
            .add_opt_value("--package", &self.package)
            .add_flag_if("--workspace", self.is_workspace)
            .add_opt_value("--exclude", &self.exclude)
    }
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Target Selection")]
pub struct CargoTargetCheckArgs {
    /// Check only this package's library
    #[clap(long = "lib", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_lib: bool,

    /// Check all binaries.
    #[clap(long = "bins", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_bins: bool,

    /// Check only the specified binary.
    #[clap(long = "bin", value_name = "NAME")]
    pub bin: Option<String>,

    /// Check all examples.
    #[clap(long = "examples", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_examples: bool,

    /// Check only the specified example.
    #[clap(long = "example", value_name = "NAME")]
    pub example: Option<String>,

    /// Check all tests.
    #[clap(long = "tests", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_tests: bool,

    /// Check only the specified test target.
    #[clap(long = "test", value_name = "NAME")]
    pub test: Option<String>,

    /// Check all benches.
    #[clap(long = "benches", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_benches: bool,

    /// Check only the specified bench target.
    #[clap(long = "bench", value_name = "NAME")]
    pub bench: Option<String>,

    /// Check all benches.
    #[clap(long = "all-targets", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_all_targets: bool,
}

impl CargoTargetCheckArgs {
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
