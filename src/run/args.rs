use clap::{ArgAction, Args, Subcommand};

use crate::external_cli::arg_builder::ArgBuilder;

#[derive(Debug, Args)]
pub struct RunArgs {
    /// The subcommands available for the run command.
    #[command(subcommand)]
    pub subcommand: Option<RunSubcommands>,

    /// Name of the bin target to run.
    #[clap(long = "bin", value_name = "NAME")]
    pub bin: Option<String>,

    /// Name of the example target to run.
    #[clap(long = "example", value_name = "NAME")]
    pub example: Option<String>,

    /// Build artifacts in release mode, with optimizations.
    #[clap(short = 'r', long = "release", action = ArgAction::SetTrue, default_value_t = false)]
    pub is_release: bool,

    /// Build for the target triple.
    #[clap(long = "target", value_name = "TRIPLE")]
    pub target: Option<String>,

    /// Directory for all generated artifacts.
    #[clap(long = "target-dir", value_name = "DIRECTORY")]
    pub target_dir: Option<String>,

    /// Path to Cargo.toml.
    #[clap(long = "manifest-path", value_name = "PATH")]
    pub manifest_path: Option<String>,
}

impl RunArgs {
    pub(crate) fn is_web(&self) -> bool {
        matches!(self.subcommand, Some(RunSubcommands::Web(_)))
    }

    /// Generate arguments for `cargo`.
    pub(crate) fn cargo_args(&self) -> ArgBuilder {
        // Web takes precedence over --target <TRIPLE>
        let target = if self.is_web() {
            Some("wasm32-unknown-unknown".to_string())
        } else {
            self.target.clone()
        };

        ArgBuilder::new()
            .add_opt_value("--bin", &self.bin)
            .add_opt_value("--example", &self.example)
            .add_flag_if("--release", self.is_release)
            .add_opt_value("--target", &target)
            .add_opt_value("--target-dir", &self.target_dir)
            .add_opt_value("--manifest-path", &self.manifest_path)
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
    pub do_open: bool,
}
