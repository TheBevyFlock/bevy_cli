use ui_test::{run_tests, CommandBuilder, Config};

fn main() -> ui_test::color_eyre::Result<()> {
    let config = config();
    run_tests(config)
}

fn config() -> Config {
    Config {
        host: None,
        root_dir: "tests/ui".into(),
        program: program(),
        out_dir: "../target/ui".into(),
        ..todo!()
    }
}

fn program() -> CommandBuilder {
    todo!()
}
