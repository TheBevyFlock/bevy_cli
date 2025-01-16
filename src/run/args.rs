use clap::{ArgAction, Args, Subcommand};

use crate::{
    build::args::{BuildArgs, BuildSubcommands, BuildWebArgs},
    external_cli::{arg_builder::ArgBuilder, cargo::run::CargoRunArgs},
};

use super::cargo::build::{CargoBuildArgs, CargoPackageBuildArgs, CargoTargetBuildArgs};

#[derive(Debug, Args)]
pub struct RunArgs {
    /// The subcommands available for the run command.
    #[command(subcommand)]
    pub subcommand: Option<RunSubcommands>,

    /// Confirm all prompts automatically.
    #[arg(long = "yes", default_value_t = false)]
    pub skip_prompts: bool,

    /// Commands to forward to `cargo run`.
    #[clap(flatten)]
    pub cargo_args: CargoRunArgs,
}

impl RunArgs {
    /// Whether to run the app in the browser.
    pub(crate) fn is_web(&self) -> bool {
        matches!(self.subcommand, Some(RunSubcommands::Web(_)))
    }

    /// Whether to build with optimizations.
    #[cfg(feature = "wasm-opt")]
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

    /// Generate arguments for `cargo`.
    pub(crate) fn cargo_args_builder(&self) -> ArgBuilder {
        self.cargo_args.args_builder(self.is_web())
    }
}

#[derive(Debug, Subcommand)]
pub enum RunSubcommands {
    /// Run your app in the browser.
    Web(RunWebArgs),
}

#[derive(Debug, Args)]
pub struct RunWebArgs {
    /// The port to run the web server on.
    #[arg(short, long, default_value_t = 4000)]
    pub port: u16,

    /// Open the app in the browser.
    #[arg(short = 'o', long = "open", action = ArgAction::SetTrue, default_value_t = false)]
    pub open: bool,

    // Bundle all web artifacts into a single folder.
    #[arg(short = 'b', long = "bundle", action = ArgAction::SetTrue, default_value_t = false)]
    pub create_packed_bundle: bool,
}

impl From<RunArgs> for BuildArgs {
    fn from(args: RunArgs) -> Self {
        BuildArgs {
            skip_prompts: args.skip_prompts,
            cargo_args: CargoBuildArgs {
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
                RunSubcommands::Web(web_args) => BuildSubcommands::Web(BuildWebArgs {
                    create_packed_bundle: web_args.create_packed_bundle,
                }),
            }),
        }
    }
}
