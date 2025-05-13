use ansi_term::Color::{Blue, Green, Purple, Red, Yellow};
use anyhow::Result;
#[cfg(feature = "rustup")]
use bevy_cli::lint::LintArgs;
use bevy_cli::{build::args::BuildArgs, run::RunArgs};
use clap::{Args, CommandFactory, Parser, Subcommand};
use tracing_subscriber::{
    fmt::{self, FormatEvent, FormatFields, format::Writer},
    prelude::*,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set default log level to info for the `bevy_cli` crate if `BEVY_LOG` is not set.
    let env = tracing_subscriber::EnvFilter::try_from_env("BEVY_LOG").map_or_else(
        |_| {
            if cli.verbose {
                tracing_subscriber::EnvFilter::new("bevy_cli=debug")
            } else {
                tracing_subscriber::EnvFilter::new("bevy_cli=info")
            }
        },
        |filter| tracing_subscriber::EnvFilter::new(format!("bevy_cli={filter}")),
    );

    let fmt_layer = fmt::layer()
        .event_format(CargoStyleFormatter)
        // enable colorized output if stderr is a terminal
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr()))
        .with_filter(env);

    tracing_subscriber::registry().with(fmt_layer).init();

    match cli.subcommand {
        Subcommands::New(new) => {
            bevy_cli::template::generate_template(&new.name, &new.template, &new.branch)?;
        }
        #[cfg(feature = "rustup")]
        Subcommands::Lint(args) => bevy_cli::lint::lint(args)?,
        Subcommands::Build(mut args) => bevy_cli::build::build(&mut args)?,
        Subcommands::Run(mut args) => bevy_cli::run::run(&mut args)?,
        Subcommands::Completions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "bevy", &mut std::io::stdout());
        }
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
    /// Use verbose output.
    ///
    /// Logs commands that are executed and more information on the actions being performed.
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,
}

/// Available subcommands for `bevy`.
#[derive(Subcommand)]
pub enum Subcommands {
    /// Create a new Bevy project from a specified template.
    New(NewArgs),
    /// Build your Bevy app.
    #[command(visible_alias = "b")]
    Build(BuildArgs),
    /// Run your Bevy app.
    #[command(visible_alias = "r")]
    Run(RunArgs),
    /// Check the current project using Bevy-specific lints.
    ///
    /// This command requires `bevy_lint` to be installed, and will fail if it is not. Please see
    /// <https://github.com/TheBevyFlock/bevy_cli> for installation instructions.
    ///
    /// To see the full list of options, run `bevy lint -- --help`.
    #[cfg(feature = "rustup")]
    Lint(LintArgs),
    /// Generate autocompletion for `bevy` CLI tool.
    ///
    /// You can add this or a variant of this to your shells `.profile` by just added
    ///
    /// ```
    /// source <(bevy completion zsh)
    /// ```
    Completions { shell: clap_complete::Shell },
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
    /// Templates are GitHub repositories. Any repository from the GitHub organization
    /// "TheBevyFlock" with the prefix `bevy_new_` will be usable via
    /// its shortcut form i.e. `2d` will use the template `bevy_new_2d`. Full GitHub URLs can also
    /// be passed in the template argument.
    ///
    /// Can be omitted to use a built-in template.
    #[arg(short, long, default_value = "minimal")]
    pub template: String,

    /// The git branch to use
    #[arg(short, long, default_value = "main")]
    pub branch: String,
}

/// Align the log formatting to match `cargo`'s style.
pub struct CargoStyleFormatter;

impl<S, N> FormatEvent<S, N> for CargoStyleFormatter
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &fmt::FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        let meta = event.metadata();

        let (color, level) = match *meta.level() {
            tracing::Level::ERROR => (Red, "error"),
            tracing::Level::WARN => (Yellow, "warning"),
            tracing::Level::INFO => (Green, "info"),
            tracing::Level::DEBUG => (Blue, "debug"),
            tracing::Level::TRACE => (Purple, "trace"),
        };

        // Apply color if desired
        let level = if writer.has_ansi_escapes() {
            color.bold().paint(level).to_string()
        } else {
            level.to_string()
        };

        write!(writer, "{level}: ",)?;
        ctx.format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}
