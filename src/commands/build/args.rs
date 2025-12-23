#[cfg(feature = "web")]
use clap::ArgAction;
use clap::{Args, Subcommand};

#[cfg(feature = "web")]
use crate::external_cli::external_cli_args::ExternalCliArgs;
#[cfg(all(feature = "unstable", feature = "web"))]
use crate::web::unstable::UnstableWebArgs;
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
    #[cfg(feature = "web")]
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

    /// The flags to use for `wasm-opt` if building for the web.
    #[cfg(feature = "web")]
    pub(crate) fn wasm_opt_args(&self) -> ExternalCliArgs {
        use crate::external_cli::external_cli_args::ExternalCliArgs;

        if let Some(BuildSubcommands::Web(web_args)) = &self.subcommand {
            ExternalCliArgs::from_raw_args(web_args.wasm_opt.clone())
        } else {
            ExternalCliArgs::Enabled(false)
        }
    }

    /// The RUSTFLAGS to pass to the `cargo` command.
    pub(crate) fn rustflags(&self) -> Option<String> {
        self.cargo_args.common_args.rustflags.clone()
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
        // An explicit `--no-default-features` takes precedence. If `--no-default-features` is not
        // passed, the config's default features is used instead.
        self.cargo_args.feature_args.is_no_default_features =
            self.cargo_args.feature_args.is_no_default_features || !config.default_features();
        self.cargo_args.common_args.rustflags = self
            .cargo_args
            .common_args
            .rustflags
            .clone()
            .or(config.rustflags());

        #[cfg(feature = "web")]
        let is_release = self.is_release();

        #[cfg(feature = "web")]
        if let Some(BuildSubcommands::Web(web_args)) = self.subcommand.as_mut() {
            if web_args.wasm_opt.is_empty() {
                web_args.wasm_opt = config.wasm_opt(is_release).to_raw();
            }

            #[cfg(feature = "unstable")]
            web_args.unstable.apply_config(config);
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
#[cfg(feature = "web")]
#[derive(Debug, Args, Default)]
pub struct BuildWebArgs {
    /// Bundle all web artifacts into a single folder.
    #[arg(short = 'b', long = "bundle", action = ArgAction::SetTrue, default_value_t = false)]
    pub create_packed_bundle: bool,
    /// Use `wasm-opt` to optimize the wasm binary
    ///
    /// Defaults to `true` for release builds.
    /// Can be set to `false` to skip optimization.
    /// You can also specify custom arguments to use.
    #[arg(long = "wasm-opt", allow_hyphen_values = true)]
    pub wasm_opt: Vec<String>,

    #[cfg(feature = "unstable")]
    #[clap(flatten)]
    pub unstable: UnstableWebArgs,
}
