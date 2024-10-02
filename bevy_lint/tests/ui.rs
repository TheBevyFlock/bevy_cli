use std::{ffi::OsString, path::Path};

use ui_test::{
    color_eyre, default_any_file_filter, run_tests_generic, status_emitter, Args, CommandBuilder,
    Config,
};

fn main() -> color_eyre::Result<()> {
    let config = config();
    run_lint_tests(config)
}

fn config() -> Config {
    Config {
        host: Some(String::new()),
        root_dir: "tests/ui".into(),
        // program: program(),
        out_dir: "../target/ui".into(),
        ..Config::dummy()
    }
}

fn program() -> CommandBuilder {
    todo!()
}

fn run_lint_tests(mut config: Config) -> color_eyre::Result<()> {
    // Parse arguments from the CLI.
    let args = Args::test()?;

    config.with_args(&args);

    run_tests_generic(
        vec![config],
        // Do not match any files.
        |path, config| {
            // We can only run Rust files within the `src/bin` folder.
            if path.extension()? == "rs" && path.parent()? == Path::new("tests/ui/src/bin") {
                return Some(default_any_file_filter(path, config));
            }

            None
        },
        // Append `--bin file_stem` to the `cargo check` command.
        |config, spanned| {
            let mut argument = OsString::from("--bin ");

            let file_stem = spanned
                .span
                .file
                .file_stem() // Like `file_name()`, but it ignores the extension.
                .expect("Attempted to test a file without a name.");

            argument.push(file_stem);

            config.program.args.push(argument);
        },
        status_emitter::Text::verbose(),
    )
}
