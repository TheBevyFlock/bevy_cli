use clap::{ArgAction, Args, Subcommand};

#[cfg(feature = "web")]
use crate::external_cli::external_cli_args::ExternalCliArgs;
use crate::{
    common_args::CommonArgs,
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

    /// Arguments shared by most commands.
    #[clap(flatten)]
    pub common_args: CommonArgs,
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
    #[cfg(not(feature = "unstable"))]
    pub(crate) fn cargo_args_builder(&self) -> ArgBuilder {
        self.cargo_args.args_builder(self.is_web())
    }

    /// Generate arguments to forward to `cargo build`.
    #[cfg(feature = "unstable")]
    pub(crate) fn cargo_args_builder(&self) -> ArgBuilder {
        // If Wasm multi-threading is enabled and a target with std is used,
        // the std needs to be rebuilt to enable multi-threading features
        let rebuild_std = self.web_multi_threading()
            && self
                .target()
                .is_some_and(|target| &target == "wasm32-unknown-unknown");

        self.cargo_args
            .args_builder(self.is_web())
            // Add the flags to rebuild std
            // Unstable, requires nightly Rust
            .add_opt_value("-Z", &rebuild_std.then_some("build-std=std,panic_abort"))
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

    /// Whether multi-threading is enabled for the web app.
    #[cfg(feature = "unstable")]
    pub(crate) fn web_multi_threading(&self) -> bool {
        self.common_args.unstable.web_multi_threading()
    }

    /// The RUSTFLAGS to pass to the `cargo` command.
    #[cfg(not(feature = "unstable"))]
    pub(crate) fn rustflags(&self) -> Option<String> {
        self.cargo_args.common_args.rustflags.clone()
    }

    /// The RUSTFLAGS to pass to the `cargo` command.
    #[cfg(feature = "unstable")]
    pub(crate) fn rustflags(&self) -> Option<String> {
        if self.common_args.unstable.web_multi_threading() {
            // Rust's default Wasm target does not support multi-threading primitives out of the box
            // They need to be enabled manually
            let multi_threading_flags = "-C target-feature=+atomics,+bulk-memory";

            if let Some(mut rustflags) = self.cargo_args.common_args.rustflags.clone() {
                rustflags += " ";
                rustflags += multi_threading_flags;
                Some(rustflags)
            } else {
                Some(multi_threading_flags.to_owned())
            }
        } else {
            self.cargo_args.common_args.rustflags.clone()
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
        if let Some(BuildSubcommands::Web(web_args)) = self.subcommand.as_mut()
            && web_args.wasm_opt.is_empty()
        {
            web_args.wasm_opt = config.wasm_opt(is_release).to_raw();
        }

        self.common_args.apply_config(config);
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
}
