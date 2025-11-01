use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use ui_test::{CommandBuilder, Config, OptWithLine, status_emitter};

// This is set by Cargo to the absolute paths of `bevy_lint` and `bevy_lint_driver`.
const LINTER_PATH: &str = env!("CARGO_BIN_EXE_bevy_lint");

fn main() {
    let linter_path = Path::new(LINTER_PATH);

    assert!(
        linter_path.is_file(),
        "`bevy_lint` could not be found at {}, make sure to build it with `cargo build -p bevy_lint --bin bevy_lint`",
        linter_path.display(),
    );

    let mut config = Config {
        // We need to specify the host tuple manually, because if we don't then `ui_test` will try
        // running `bevy_lint -vV` to discover the host and promptly error because `bevy_lint`
        // doesn't recognize the `-vV` flag.
        host: Some(host_tuple()),
        program: CommandBuilder {
            program: linter_path.into(),
            args: vec!["--color=never".into(), "--quiet".into()],
            out_dir_flag: Some("--target-dir".into()),
            input_file_flag: Some("--manifest-path".into()),
            envs: Vec::new(),
            cfg_flag: None,
        },
        out_dir: PathBuf::from("../target/ui"),
        ..Config::cargo(Path::new("tests/ui-cargo"))
    };

    let defaults = config.comment_defaults.base();
    defaults.require_annotations = None.into();

    // Create the `#@exit-status: CODE` annotation. This can be used to ensure a UI test exits with
    // a specific exit code (e.g. `bevy_lint` exits with code 101 when a denied lint is found).
    config
        .custom_comments
        .insert("exit-status", |parser, args, _span| {
            parser.exit_status = OptWithLine::new(
                args.content
                    .parse()
                    .expect("expected `i32` as input for `exit-status`"),
                args.span,
            );
        });

    // Run this `Config` for all paths that end with `Cargo.toml` resulting
    // only in the `Cargo` lints.
    ui_test::run_tests_generic(
        vec![config],
        |path, config| {
            path.ends_with("Cargo.toml")
                .then(|| ui_test::default_any_file_filter(path, config))
        },
        |_config, _file_contents| {},
        status_emitter::Text::verbose(),
    )
    .unwrap();
}

/// Queries the host tuple from `rustc` and returns it as a string.
fn host_tuple() -> String {
    let output = Command::new("rustc")
        .arg("--print=host-tuple")
        // Show errors directly to the user, rather than capturing them.
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run `rustc --print=host-tuple`");

    // `rustc` only works with UTF-8, so it's safe to error if invalid UTF-8 is found.
    str::from_utf8(&output.stdout)
        .expect("`rustc --print=host-tuple` did not emit valid UTF-8")
        // Remove the trailing `\n`.
        .trim_end()
        .to_string()
}
