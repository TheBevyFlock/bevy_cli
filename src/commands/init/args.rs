#[derive(Debug, Args)]
pub struct InitArgs {
    /// The subcommands available for the init command.
    #[clap(subcommand)]
    pub subcommand: Option<InitSubcommands>,
}

#[derive(Debug, Subcommand)]
pub enum InitSubcommands {
    FastCompiles(FastCompilesArgs),
}

pub struct FastCompilesArgs {
    /// Enable fast compiles for the project.
    #[arg(long = "enable", action = ArgAction::SetTrue)]
    pub enable: bool,
}
