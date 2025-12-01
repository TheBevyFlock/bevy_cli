use clap::{Args, Subcommand};

#[cfg(feature = "rustup")]
use crate::external_cli::cargo::install::AutoInstall;
use crate::{
    config::CliConfig,
    external_cli::{arg_builder::ArgBuilder, cargo::check::CargoCheckArgs},
};

#[derive(Debug, Args)]
pub struct LintArgs {
    #[clap(subcommand)]
    pub subcommand: Option<LintSubcommands>,

    /// Show version information
    #[arg(long = "version", default_value_t = false)]
    pub version: bool,

    /// Automatically fix lint warnings reported by bevy_lint.
    #[arg(long = "fix", default_value_t = false)]
    pub fix: bool,

    /// Arguments to forward to `cargo check`.
    #[clap(flatten)]
    pub cargo_args: CargoCheckArgs,

    /// Arguments to forward to `bevy_lint`
    ///
    /// Specified after `--`.
    #[clap(last = true, name = "ARGS", global = true)]
    pub forward_args: Vec<String>,
}

impl LintArgs {
    /// Determine if the app is being built for the web.
    #[cfg(feature = "web")]
    pub(crate) fn is_web(&self) -> bool {
        matches!(self.subcommand, Some(LintSubcommands::Web))
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

    /// The targeted platform.
    pub(crate) fn target(&self) -> Option<String> {
        self.cargo_args.compilation_args.target(self.is_web())
    }

    /// Generate arguments to forward to `cargo check`.
    pub(crate) fn cargo_args_builder(&self) -> ArgBuilder {
        self.cargo_args.args_builder(self.is_web())
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
    }
}

/// The subcommands available for the lint command.
#[derive(Debug, Subcommand)]
pub enum LintSubcommands {
    /// Lint your app for the browser.
    #[cfg(feature = "web")]
    Web,
    /// Install a `bevy_lint` version.
    #[cfg(feature = "rustup")]
    Install(InstallArgs),
    /// List all available `bevy_lint` versions.
    List,
}

#[derive(Debug, Args)]
pub struct InstallArgs {
    pub version: Option<String>,

    /// Confirm all prompts automatically.
    #[arg(long = "yes", default_value_t = false)]
    pub confirm_prompts: bool,
}

impl InstallArgs {
    /// Whether to automatically install missing dependencies.
    #[cfg(feature = "rustup")]
    pub(crate) fn auto_install(&self) -> AutoInstall {
        if self.confirm_prompts {
            AutoInstall::Always
        } else {
            AutoInstall::AskUser
        }
    }
}
