use anyhow::Result;
use clap::{Args, Parser, Subcommand};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.subcommand {
        Subcommands::New(new) => {
            bevy_cli::template::generate_template(&new.name, new.template.as_deref())?;
        }
        Subcommands::Lint => bevy_cli::lint::lint()?,
    }

    Ok(())
}

/// Command-line interface for the Bevy Game Engine
///
/// This CLI provides tools for Bevy project management,
/// such as generating new projects from templates.
#[derive(Parser)]
#[command(name = "bevy", version, about, next_line_help(false))]
pub struct Cli {
    /// Available subcommands for the Bevy CLI.
    #[command(subcommand)]
    pub subcommand: Subcommands,
}

/// Available subcommands for `bevy`.
#[derive(Subcommand)]
pub enum Subcommands {
    /// Create a new Bevy project from a specified template.
    New(NewArgs),
    /// Check the current project using Bevy-specific lints.
    Lint,
}

/// Arguments for creating a new Bevy project.
///
/// This subcommand allows you to generate a new Bevy project
/// using a specified template and project name.
#[derive(Args)]
pub struct NewArgs {
    /// The desired name for the new project.
    ///
    /// This will be the name of the directory and will be used in the project's files
    pub name: String,

    /// The name of the template to use for generating the project.
    ///
    /// This can be a GitHub repository (`user/repo`) or a full Git URL.
    ///
    /// Can be omitted to use a built-in template.
    #[arg(short, long)]
    pub template: Option<String>,
}
