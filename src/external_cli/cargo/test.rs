use clap::{ArgAction, Args};

use crate::external_cli::{CommandExt, arg_builder::ArgBuilder};

use super::{CargoCommonArgs, CargoCompilationArgs, CargoFeatureArgs, CargoManifestArgs, program};

/// Create a command to run `cargo test`.
pub(crate) fn command(test_name: Option<&str>) -> CommandExt {
    let mut command = CommandExt::new(program());
    command.arg("test");
    if let Some(test_name) = test_name {
        command.arg(test_name);
    }
    command
}

#[derive(Debug, Args)]
pub struct CargoTestArgs {
    #[clap(flatten)]
    pub common_args: CargoCommonArgs,
    #[clap(flatten)]
    pub package_args: CargoPackageTestArgs,
    #[clap(flatten)]
    pub target_args: CargoTargetTestArgs,
    #[clap(flatten)]
    pub feature_args: CargoFeatureArgs,
    #[clap(flatten)]
    pub compilation_args: CargoCompilationArgs,
    #[clap(flatten)]
    pub manifest_args: CargoManifestArgs,
}

impl CargoTestArgs {
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
pub struct CargoPackageTestArgs {
    /// Package to run tests for
    #[clap(short = 'p', long = "package", value_name = "SPEC")]
    pub package: Option<String>,

    /// Test all packages in the workspace
    #[clap(long = "workspace", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_workspace: bool,

    /// Exclude packages from the test
    #[clap(long = "exclude", value_name = "SPEC")]
    pub exclude: Option<String>,
}

impl CargoPackageTestArgs {
    pub(crate) fn args_builder(&self) -> ArgBuilder {
        ArgBuilder::new()
            .add_opt_value("--package", &self.package)
            .add_flag_if("--workspace", self.is_workspace)
            .add_opt_value("--exclude", &self.exclude)
    }
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Target Selection")]
pub struct CargoTargetTestArgs {
    /// Test only this package's library
    #[clap(long = "lib", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_lib: bool,

    /// Test all binaries
    #[clap(long = "bins", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_bins: bool,

    /// Test only the specified binary
    #[clap(long = "bin", value_name = "NAME")]
    pub bin: Option<String>,

    /// Test all examples.
    #[clap(long = "examples", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_examples: bool,

    /// Test only the specified example.
    #[clap(long = "example", value_name = "NAME")]
    pub example: Option<String>,

    /// Test all targets that have `test = true` set
    #[clap(long = "tests", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_tests: bool,

    /// Test only the specified test target
    #[clap(long = "test", value_name = "NAME")]
    pub test: Option<String>,

    /// Test all targets that have `bench = true` set
    #[clap(long = "benches", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_benches: bool,

    /// Test only the specified bench target.
    #[clap(long = "bench", value_name = "NAME")]
    pub bench: Option<String>,

    /// Test all targets (does not include doctests)
    #[clap(long = "all-targets", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_all_targets: bool,

    /// Test only this library's documentation
    #[clap(long = "doc", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_doc: bool,
}

impl CargoTargetTestArgs {
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
            .add_flag_if("--doc", self.is_doc)
    }
}
