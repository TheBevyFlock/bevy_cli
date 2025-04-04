use clap::{ArgAction, Args, Subcommand};

use crate::{
    config::CliConfig,
    external_cli::{arg_builder::ArgBuilder, cargo::build::CargoBuildArgs},
};

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// The subcommands available for the build command.
    #[clap(subcommand)]
    pub subcommand: Option<BuildSubcommands>,

    /// Confirm all prompts automatically.
    #[arg(long = "yes", default_value_t = false)]
    pub skip_prompts: bool,

    /// Arguments to forward to `cargo build`.
    #[clap(flatten)]
    pub cargo_args: CargoBuildArgs,
}

impl BuildArgs {
    /// Determine if the app is being built for the web.
    #[cfg(feature = "web")]
    pub(crate) fn is_web(&self) -> bool {
        matches!(self.subcommand, Some(BuildSubcommands::Web(_)))
    }
    #[cfg(not(feature = "web"))]
    pub(crate) fn is_web(&self) -> bool {
        false
    }

    /// Whether to build with optimizations.
    pub(crate) fn is_release(&self) -> bool {
        self.cargo_args.compilation_args.is_release
    }

    /// The profile used to compile the app.
    pub(crate) fn profile(&self) -> &str {
        self.cargo_args.compilation_args.profile(self.is_web())
    }

    /// The targeted platform.
    pub(crate) fn target(&self) -> Option<String> {
        self.cargo_args.compilation_args.target(self.is_web())
    }

    /// Generate arguments to forward to `cargo build`.
    pub(crate) fn cargo_args_builder(&self) -> ArgBuilder {
        self.cargo_args.args_builder(self.is_web())
    }

    /// Apply the config on top of the CLI arguments.
    ///
    /// CLI arguments take precedence.
    pub(crate) fn apply_config(&mut self, config: &CliConfig) {
        self.cargo_args
            .feature_args
            .features
            .extend(config.features().iter().cloned());
        self.cargo_args.feature_args.is_no_default_features = Some(
            self.cargo_args
                .feature_args
                .is_no_default_features
                .unwrap_or(!config.default_features()),
        );
    }
}

#[derive(Debug, Subcommand)]
pub enum BuildSubcommands {
    /// Build your app for the browser.
    #[cfg(feature = "web")]
    Web(BuildWebArgs),
}

#[derive(Debug, Args)]
pub struct BuildWebArgs {
    // Bundle all web artifacts into a single folder.
    #[arg(short = 'b', long = "bundle", action = ArgAction::SetTrue, default_value_t = false)]
    pub create_packed_bundle: bool,
}
