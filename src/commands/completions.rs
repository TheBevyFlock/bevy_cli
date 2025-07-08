use clap::Parser;
use clap_complete::Shell;

/// Generate code for auto-completions for the given shell.
pub fn completions<Cli: Parser>(shell: Shell) {
    clap_complete::generate(shell, &mut Cli::command(), "bevy", &mut std::io::stdout());
}
