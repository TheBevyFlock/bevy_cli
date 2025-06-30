use clap::Parser;
use clap_complete::Shell;

pub fn completions<Cli: Parser>(shell: Shell) {
    clap_complete::generate(shell, &mut Cli::command(), "bevy", &mut std::io::stdout());
}
