use std::{fmt::Write, process::ExitCode};

use ansi_term::Color::{Blue, Green, Purple, Red, Yellow};
#[cfg(feature = "rustup")]
use bevy_cli::commands::lint::{LintArgs, lint};
use bevy_cli::commands::{
    build::{BuildArgs, build},
    completions::completions,
    run::{RunArgs, run},
};
use clap::{Args, Parser, Subcommand, builder::styling::Style};
use clap_cargo::style;
use tracing::error;
use tracing_subscriber::{
    fmt::{self, FormatEvent, FormatFields, format::Writer},
    prelude::*,
};

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Set default log level to info for the `bevy_cli` crate if `BEVY_LOG` is not set.
    let env = tracing_subscriber::EnvFilter::try_from_env("BEVY_LOG").map_or_else(
        |_| {
            if cli.verbose {
                tracing_subscriber::EnvFilter::new("bevy_cli=debug,bevy_cli_bin=debug")
            } else {
                tracing_subscriber::EnvFilter::new("bevy_cli=info,bevy_cli_bin=info")
            }
        },
        |filter| {
            tracing_subscriber::EnvFilter::new(format!("bevy_cli={filter},bevy_cli_bin={filter}"))
        },
    );

    let fmt_layer = fmt::layer()
        .event_format(CargoStyleFormatter)
        // enable colorized output if stderr is a terminal
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr()))
        .with_filter(env);

    tracing_subscriber::registry().with(fmt_layer).init();

    if let Err(error) = match cli.subcommand {
        Subcommands::New(new) => {
            bevy_cli::template::generate_template(&new.name, &new.template, &new.branch).map(|_| ())
        }
        #[cfg(feature = "rustup")]
        Subcommands::Lint(args) => lint(args),
        Subcommands::Build(mut args) => build(&mut args),
        Subcommands::Run(mut args) => run(&mut args),
        Subcommands::Completions { shell } => {
            completions::<Cli>(shell);
            Ok(())
        }
    } {
        if cli.verbose {
            // `anyhow::Error`'s `Debug` implementation prints backtraces, while `Display` does not.
            error!(target:"bevy_cli_bin", "{error:?}");
        } else {
            error!(target:"bevy_cli_bin", "{error}");
        }
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

/// Command-line interface for the Bevy Game Engine
///
/// This CLI provides tools for Bevy project management,
/// such as generating new projects from templates.
#[derive(Parser)]
#[command(name = "bevy", version, about, next_line_help(false), after_help = after_help(), styles = style::CLAP_STYLING)]
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

fn after_help() -> String {
    let mut message = String::new();

    let header = style::HEADER;
    let literal = style::LITERAL;
    let underline = Style::new().underline();

    _ = writeln!(message, "{header}Resources:{header:#}");
    _ = writeln!(
        message,
        "  {literal}Bevy Website{literal:#}       {underline}https://bevyengine.org{underline:#}"
    );
    _ = writeln!(
        message,
        "  {literal}Bevy Repository{literal:#}    {underline}https://github.com/bevyengine/bevy{underline:#}"
    );
    _ = writeln!(
        message,
        "  {literal}CLI Documentation{literal:#}  {underline}https://thebevyflock.github.io/bevy_cli{underline:#}"
    );

    message
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
    #[command(after_help = run_after_help())]
    Run(RunArgs),
    /// Check the current project using Bevy-specific lints.
    ///
    /// To see the full list of options, run `bevy lint -- --help`.
    #[cfg(feature = "rustup")]
    #[command(after_help = lint_after_help())]
    Lint(LintArgs),
    /// Prints the auto-completion script for a specific shell.
    ///
    /// The result of this command is intended to be passed to the `source` command, such as
    /// `source <(bevy completions $SHELL_NAME)`. (Please see the examples for more details.) You
    /// likely want to run this whenever your shell is started by adding the command to `.profile`,
    /// `.bashrc`, or another startup script.
    #[command(after_help = completions_after_help())]
    Completions { shell: clap_complete::Shell },
}

fn run_after_help() -> String {
    let mut message = String::new();

    let header = style::HEADER;
    let literal = style::LITERAL;
    let placeholder = style::PLACEHOLDER;

    _ = writeln!(message, "{header}Examples:{header:#}");
    _ = writeln!(message, "  {literal}bevy run{literal:#}");
    _ = writeln!(message, "  {literal}bevy run web{literal:#}");
    _ = writeln!(
        message,
        "  {literal}bevy run --example{literal:#} {placeholder}<NAME>{placeholder:#} {literal}web{literal:#}",
    );

    message
}

#[cfg(feature = "rustup")]
fn lint_after_help() -> String {
    let mut message = String::new();

    let header = style::HEADER;
    let literal = style::LITERAL;

    _ = writeln!(message, "{header}Examples:{header:#}");
    _ = writeln!(message, "  {literal}bevy lint{literal:#}");
    _ = writeln!(
        message,
        "  {literal}bevy lint --all-features --all-targets{literal:#}"
    );

    message
}

fn completions_after_help() -> String {
    let mut message = String::new();

    let header = style::HEADER;
    let literal = style::LITERAL;

    _ = writeln!(message, "{header}Examples:{header:#}");
    _ = writeln!(
        message,
        "  {literal}source <(bevy completions bash){literal:#}"
    );
    _ = writeln!(
        message,
        "  {literal}source <(bevy completions zsh){literal:#}"
    );

    message
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
