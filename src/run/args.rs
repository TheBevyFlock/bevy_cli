use clap::{ArgAction, Args, Subcommand};

#[cfg(feature = "web")]
use crate::build::args::{BuildSubcommands, BuildWebArgs};
use crate::{
    build::args::BuildArgs,
    config::CliConfig,
    external_cli::{
        arg_builder::ArgBuilder,
        cargo::{install::AutoInstall, run::CargoRunArgs},
    },
};

use super::cargo::build::{CargoBuildArgs, CargoPackageBuildArgs, CargoTargetBuildArgs};

#[derive(Debug, Args, Clone)]
pub struct RunArgs {
    /// The subcommands available for the run command.
    #[command(subcommand)]
    pub subcommand: Option<RunSubcommands>,

    /// Confirm all prompts automatically.
    #[arg(long = "yes", default_value_t = false)]
    pub confirm_prompts: bool,

    /// Commands to forward to `cargo run`.
    #[clap(flatten)]
    pub cargo_args: CargoRunArgs,

    /// Arguments to pass to the underlying Bevy app.
    ///
    /// Specified after `--`.
    #[clap(last = true, name = "ARGS")]
    pub forward_args: Vec<String>,
}

impl RunArgs {
    /// Whether to automatically install missing dependencies.
    pub(crate) fn auto_install(&self) -> AutoInstall {
        if self.confirm_prompts {
            AutoInstall::Always
        } else {
            AutoInstall::AskUser
        }
    }

    /// Whether to run the app in the browser.
    #[cfg(feature = "web")]
    pub(crate) fn is_web(&self) -> bool {
        matches!(self.subcommand, Some(RunSubcommands::Web(_)))
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

    /// Generate arguments for `cargo`.
    pub(crate) fn cargo_args_builder(&self) -> ArgBuilder {
        let mut arg_builder = self.cargo_args.args_builder(self.is_web());

        if !self.forward_args.is_empty() {
            // This MUST come last to avoid forwarding other args
            arg_builder = arg_builder.arg("--").args(self.forward_args.iter());
        }

        arg_builder
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
        if let Some(RunSubcommands::Web(web_args)) = self.subcommand.as_mut() {
            if web_args.use_wasm_opt.is_none() {
                web_args.use_wasm_opt = config.wasm_opt();
            }
        }
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum RunSubcommands {
    /// Run your app in the browser.
    #[cfg(feature = "web")]
    Web(RunWebArgs),
}

#[derive(Debug, Args, Clone)]
pub struct RunWebArgs {
    /// The port to run the web server on.
    #[arg(short, long, default_value_t = 4000)]
    pub port: u16,

    /// Open the app in the browser.
    #[arg(short = 'o', long = "open", action = ArgAction::SetTrue, default_value_t = false)]
    pub open: bool,

    /// Bundle all web artifacts into a single folder.
    #[arg(short = 'b', long = "bundle", action = ArgAction::SetTrue, default_value_t = false)]
    pub create_packed_bundle: bool,

    /// Headers to add to the web-server responses, in the format `name:value` or `name=value`.
    ///
    /// Can be defined multiple times to add multiple headers.
    #[clap(short = 'H', long = "headers", value_name = "HEADERS")]
    pub headers: Vec<String>,

    // Use `wasm-opt` to optimize the wasm binary
    #[arg(long = "wasm-opt")]
    pub use_wasm_opt: Option<bool>,
}

impl Default for RunWebArgs {
    fn default() -> Self {
        Self {
            port: 4000,
            open: false,
            create_packed_bundle: false,
            headers: Vec::new(),
            use_wasm_opt: None,
        }
    }
}

impl From<RunArgs> for BuildArgs {
    fn from(args: RunArgs) -> Self {
        BuildArgs {
            confirm_prompts: args.confirm_prompts,
            cargo_args: CargoBuildArgs {
                common_args: args.cargo_args.common_args,
                compilation_args: args.cargo_args.compilation_args,
                feature_args: args.cargo_args.feature_args,
                manifest_args: args.cargo_args.manifest_args,
                package_args: CargoPackageBuildArgs {
                    package: args.cargo_args.package_args.package,
                    is_workspace: false,
                    exclude: None,
                },
                target_args: CargoTargetBuildArgs {
                    bin: args.cargo_args.target_args.bin,
                    example: args.cargo_args.target_args.example,
                    is_all_targets: false,
                    is_benches: false,
                    is_bins: false,
                    is_examples: false,
                    is_lib: false,
                    is_tests: false,
                    bench: None,
                    test: None,
                },
            },
            subcommand: args.subcommand.map(|subcommand| match subcommand {
                #[cfg(feature = "web")]
                RunSubcommands::Web(web_args) => BuildSubcommands::Web(BuildWebArgs {
                    create_packed_bundle: web_args.create_packed_bundle,
                    use_wasm_opt: web_args.use_wasm_opt,
                }),
            }),
        }
    }
}
