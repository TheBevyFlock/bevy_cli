use clap::{Args, Subcommand};

use crate::{
    config::CliConfig,
    external_cli::{
        arg_builder::ArgBuilder,
        cargo::{install::AutoInstall, test::CargoTestArgs},
    },
};

/// Arguments for testing a Bevy project.
#[derive(Debug, Args)]
pub struct TestArgs {
    /// The subcommands available for the test command.
    #[clap(subcommand)]
    pub subcommand: Option<TestSubcommands>,
    /// If specified, only run tests containing this string in their names
    pub test_name: Option<String>,
    /// Arguments to forward to `cargo test`.
    #[clap(flatten)]
    pub cargo_args: CargoTestArgs,

    /// Arguments to pass to the tests.
    ///
    /// Specified after `--`.
    #[clap(last = true, name = "ARGS")]
    pub forward_args: Vec<String>,
}

impl TestArgs {
    // There is nothing to install yet for tests
    pub(crate) fn auto_install() -> AutoInstall {
        AutoInstall::Never
    }

    /// Determine if the app is being built for the web.
    pub(crate) fn is_web(&self) -> bool {
        matches!(self.subcommand, Some(TestSubcommands::Web))
            || self.cargo_args.compilation_args.profile.as_deref() == Some("web-release")
            || self.cargo_args.compilation_args.profile.as_deref() == Some("web")
    }
    /// Whether to test in release mode.
    pub(crate) fn is_release(&self) -> bool {
        self.cargo_args.compilation_args.profile.as_deref() == Some("release")
            || self.cargo_args.compilation_args.profile.as_deref() == Some("web-release")
            || self.cargo_args.compilation_args.is_release
    }

    /// The profile used to test the app.
    pub(crate) fn profile(&self) -> &str {
        self.cargo_args.compilation_args.profile(self.is_web())
    }

    /// The targeted platform.
    pub(crate) fn target(&self) -> Option<String> {
        self.cargo_args.compilation_args.target(self.is_web())
    }

    /// Generate arguments to forward to `cargo test`.
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
    }
}

/// The subcommands available for the test command.
#[derive(Debug, Subcommand)]
pub enum TestSubcommands {
    /// Test your app for the browser.
    Web,
}
