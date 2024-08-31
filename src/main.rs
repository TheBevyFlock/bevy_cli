use anyhow::Result;
use args::{Cli, Subcommands};
use cargo_generate::GenerateArgs;
use clap::Parser as _;

mod args;
mod build;
mod external_cli;
mod files;
mod lint;
mod mainfest;
mod run;
mod web;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.subcommand {
        Subcommands::New(new) => {
            cargo_generate::generate(GenerateArgs {
                template_path: new.template_path(),
                name: Some(new.name),
                force: true, // prevent conversion to kebab-case
                ..Default::default()
            })?;
        }
        Subcommands::Build(args) => build::build(&args)?,
        Subcommands::Run(args) => run::run(&args)?,
        Subcommands::Lint => lint::lint()?,
    }

    Ok(())
}
