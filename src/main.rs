use anyhow::Result;
use args::{Cli, Subcommands};
use cargo_generate::GenerateArgs;
use clap::Parser as _;

mod args;

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
    }

    Ok(())
}
