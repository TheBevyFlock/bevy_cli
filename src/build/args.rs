use clap::{ArgAction, Args, Subcommand};

use crate::{
    config::CliConfig,
    external_cli::{
        arg_builder::ArgBuilder,
        cargo::{build::CargoBuildArgs, install::AutoInstall},
    },
};

/// Arguments for building a Bevy project.
#[derive(Debug, Args)]
pub struct BuildArgs {
    /// The subcommands available for the build command.
    #[clap(subcommand)]
    pub subcommand: Option<BuildSubcommands>,

    /// Confirm all prompts automatically.
    #[arg(long = "yes", default_value_t = false)]
    pub confirm_prompts: bool,

    /// Arguments to forward to `cargo build`.
    #[clap(flatten)]
    pub cargo_args: CargoBuildArgs,
}

impl BuildArgs {
    /// Whether to automatically install missing dependencies.
    pub(crate) fn auto_install(&self) -> AutoInstall {
        if self.confirm_prompts {
            AutoInstall::Always
        } else {
            AutoInstall::AskUser
        }
    }

    /// Determine if the app is being built for the web.
    #[cfg(feature = "web")]
    pub(crate) fn is_web(&self) -> bool {
        matches!(self.subcommand, Some(BuildSubcommands::Web(_)))
            || self.cargo_args.compilation_args.profile.as_deref() == Some("web-release")
            || self.cargo_args.compilation_args.profile.as_deref() == Some("web")
    }
    #[cfg(not(feature = "web"))]
    pub(crate) fn is_web(&self) -> bool {
        false
    }

    /// Whether to build with optimizations.
    pub(crate) fn is_release(&self) -> bool {
        self.cargo_args.compilation_args.profile.as_deref() == Some("release")
            || self.cargo_args.compilation_args.profile.as_deref() == Some("web-release")
            || self.cargo_args.compilation_args.is_release
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

    /// Whether to use `wasm-opt`.
    ///
    /// Defaults to `true` for release builds.
    #[cfg(feature = "web")]
    pub(crate) fn use_wasm_opt(&self) -> bool {
        if let Some(BuildSubcommands::Web(web_args)) = &self.subcommand {
            web_args.use_wasm_opt.map_or(self.is_release(), |v| v)
        } else {
            false
        }
    }

    /// Apply the config on top of the CLI arguments.
    ///
    /// CLI arguments take precedence.
    pub(crate) fn apply_config(&mut self, config: &CliConfig) {
        if config.is_default() {
            return;
        }

        tracing::debug!("using defaults from bevy_cli config:\n{config}");
        if self.cargo_args.compilation_args.target.is_none() {
            self.cargo_args.compilation_args.target = config.target().map(ToOwned::to_owned);
        }
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
        self.cargo_args.common_args.rustflags = self
            .cargo_args
            .common_args
            .rustflags
            .clone()
            .or(config.rustflags());

        #[cfg(feature = "web")]
        if let Some(BuildSubcommands::Web(web_args)) = self.subcommand.as_mut() {
            if web_args.use_wasm_opt.is_none() {
                web_args.use_wasm_opt = config.wasm_opt();
            }
        }
    }
}

/// The subcommands available for the build command.
#[derive(Debug, Subcommand)]
pub enum BuildSubcommands {
    /// Build your app for the browser.
    #[cfg(feature = "web")]
    Web(BuildWebArgs),
}

/// Additional Arguments for building a Bevy web project.
#[derive(Debug, Args, Default)]
pub struct BuildWebArgs {
    // Bundle all web artifacts into a single folder.
    #[arg(short = 'b', long = "bundle", action = ArgAction::SetTrue, default_value_t = false)]
    pub create_packed_bundle: bool,
    // Use `wasm-opt` to optimize the wasm binary
    #[arg(long = "wasm-opt")]
    pub use_wasm_opt: Option<bool>,
}
