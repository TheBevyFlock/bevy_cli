use clap::{Args, Subcommand};

use crate::external_cli::{arg_builder::ArgBuilder, cargo::build::CargoBuildArgs};

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// The subcommands available for the build command.
    #[clap(subcommand)]
    pub subcommand: Option<BuildSubcommands>,

    /// Arguments to forward to `cargo build`.
    #[clap(flatten)]
    pub cargo_args: CargoBuildArgs,
}

impl BuildArgs {
    /// Determine if the app is being built for the web.
    pub(crate) fn is_web(&self) -> bool {
        matches!(self.subcommand, Some(BuildSubcommands::Web))
    }

    /// Whether the app is built with optimizations enabled.
    pub(crate) fn is_release(&self) -> bool {
        self.cargo_args.compilation_args.is_release
    }

    /// Generate arguments for `cargo`.
    pub(crate) fn cargo_args_builder(&self) -> ArgBuilder {
        let args = &self.cargo_args;

        ArgBuilder::new()
            .append(args.package_args.args_builder())
            .append(args.target_args.args_builder())
            .append(args.feature_args.args_builder())
            .append(args.compilation_args.args_builder(self.is_web()))
            .append(args.manifest_args.args_builder())
    }
}

#[derive(Debug, Subcommand)]
pub enum BuildSubcommands {
    /// Build your app for the browser.
    Web,
}
