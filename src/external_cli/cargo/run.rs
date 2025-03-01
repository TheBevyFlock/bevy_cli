use clap::Args;

use crate::external_cli::{arg_builder::ArgBuilder, Command};

use super::{program, CargoCommonArgs, CargoCompilationArgs, CargoFeatureArgs, CargoManifestArgs};

/// Create a command to run `cargo run`.
pub(crate) fn command() -> Command {
    let mut command = Command::new(program());
    command.arg("run");
    command
}

#[derive(Debug, Args, Clone)]
pub struct CargoRunArgs {
    #[clap(flatten)]
    pub common_args: CargoCommonArgs,
    #[clap(flatten)]
    pub package_args: CargoPackageRunArgs,
    #[clap(flatten)]
    pub target_args: CargoTargetRunArgs,
    #[clap(flatten)]
    pub feature_args: CargoFeatureArgs,
    #[clap(flatten)]
    pub compilation_args: CargoCompilationArgs,
    #[clap(flatten)]
    pub manifest_args: CargoManifestArgs,
}

impl CargoRunArgs {
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

#[derive(Debug, Args, Clone)]
#[command(next_help_heading = "Package Selection")]
pub struct CargoPackageRunArgs {
    /// Package with the target to run
    #[clap(short = 'p', long = "package", value_name = "SPEC")]
    pub package: Option<String>,
}

impl CargoPackageRunArgs {
    pub(crate) fn args_builder(&self) -> ArgBuilder {
        ArgBuilder::new().add_opt_value("--package", &self.package)
    }
}

#[derive(Debug, Args, Clone)]
#[command(next_help_heading = "Target Selection")]
pub struct CargoTargetRunArgs {
    /// Build only the specified binary.
    #[clap(long = "bin", value_name = "NAME")]
    pub bin: Option<String>,

    /// Build only the specified example.
    #[clap(long = "example", value_name = "NAME")]
    pub example: Option<String>,
}

impl CargoTargetRunArgs {
    pub(crate) fn args_builder(&self) -> ArgBuilder {
        ArgBuilder::new()
            .add_opt_value("--bin", &self.bin)
            .add_opt_value("--example", &self.example)
    }
}
