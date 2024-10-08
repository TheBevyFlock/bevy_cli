// A convenience feature used in `find_bevy_rlib()`.
#![feature(let_chains)]

use serde::Deserialize;
use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use ui_test::{
    color_eyre::{self, eyre::ensure},
    run_tests, CommandBuilder, Config,
};

// This is set by `build.rs`. It is the version specified in `rust-toolchain.toml`.
const RUST_TOOLCHAIN_CHANNEL: &str = env!("RUST_TOOLCHAIN_CHANNEL");

fn main() -> color_eyre::Result<()> {
    let config = config()?;
    run_tests(config)
}

/// Generates a custom [`Config`] for `bevy_lint`'s UI tests.
fn config() -> color_eyre::Result<Config> {
    const DRIVER_STEM: &str = "../target/debug/bevy_lint_driver";

    // The path to the `bevy_lint_driver` executable, relative from inside the `bevy_lint` folder.
    // We use `with_extension()` to potentially add the `.exe` suffix, if on Windows.
    let driver_path = Path::new(DRIVER_STEM).with_extension(env::consts::EXE_EXTENSION);

    ensure!(
        driver_path.is_file(),
        "`bevy_lint_driver` could not be found at {}, make sure to build it with `cargo build -p bevy_lint --bin bevy_lint_driver`.",
        driver_path.display(),
    );

    let config = Config {
        // When `host` is `None`, `ui_test` will attempt to auto-discover the host by calling
        // `program -vV`. Unfortunately, `bevy_lint_driver` does not yet support the version flag,
        // so we manually specify the host as an empty string. This means that, for now, host-
        // specific configuration in UI tests will not work.
        host: Some(String::new()),
        program: CommandBuilder {
            // We call `rustup run` to setup the proper environmental variables, so that
            // `bevy_lint_driver` can link to `librustc_driver.so`.
            program: "rustup".into(),
            args: vec![
                "run".into(),
                RUST_TOOLCHAIN_CHANNEL.into(),
                driver_path.into(),
                // `bevy_lint_driver` expects the first argument to be the path to `rustc`.
                "rustc".into(),
                // This is required so that `ui_test` can parse warnings and errors.
                "--error-format=json".into(),
                // These two lines tell `rustc` to search in `target/debug/deps` for dependencies.
                // This is required for UI tests to import `bevy`.
                "-L".into(),
                "all=../target/debug/deps".into(),
                // Make the `bevy` crate directly importable from the UI tests.
                format!("--extern=bevy={}", find_bevy_rlib()?.display()).into(),
            ],
            out_dir_flag: Some("--out-dir".into()),
            input_file_flag: None,
            envs: Vec::new(),
            cfg_flag: Some("--print=cfg".into()),
        },
        out_dir: PathBuf::from("../target/ui"),
        ..Config::rustc("tests/ui")
    };

    Ok(config)
}

/// An artifact message printed to stdout by Cargo.
///
/// This only deserializes the fields necessary to run UI tests, the rest of skipped.
///
/// See <https://doc.rust-lang.org/cargo/reference/external-tools.html#artifact-messages> for more
/// information on the exact format.
#[derive(Deserialize, Debug)]
#[serde(rename = "compiler-artifact", tag = "reason")]
struct ArtifactMessage<'a> {
    package_id: &'a str,
    target: ArtifactTarget<'a>,
    filenames: Vec<&'a Path>,
}

/// The `"target"` field of an [`ArtifactMessage`].
#[derive(Deserialize, Debug)]
struct ArtifactTarget<'a> {
    name: &'a str,
    kind: Vec<&'a str>,
}

/// Tries to find the path to `libbevy.rlib` that UI tests import.
///
/// `bevy` is a dev-dependency, and as such is only built for tests and examples. We can force it
/// to be built by calling `cargo build --test=ui --message-format=json`, then scan the printed
/// JSON for the artifact message with the path to `libbevy.rlib`.
///
/// The reason we specify `--extern bevy=PATH` instead of just `--extern bevy` is because `rustc`
/// will fail to compile if multiple `libbevy.rlib` files are found, which usually is the case.
fn find_bevy_rlib() -> color_eyre::Result<PathBuf> {
    // `bevy` is a dev-dependency, so building a test will require it to be built as well.
    let output = Command::new("cargo")
        .arg("build")
        .arg("--test=ui")
        .arg("--message-format=json")
        // Show error messages to the user for easier debugging.
        .stderr(Stdio::inherit())
        .output()?;

    ensure!(output.status.success());

    const NEWLINE: u8 = '\n' as u8;
    const BEVY_PACKAGE_ID_PREFIX: &str =
        "registry+https://github.com/rust-lang/crates.io-index#bevy@";

    let mut messages = Vec::with_capacity(1);

    for line in output.stdout.split(|&byte| byte == NEWLINE) {
        if let Ok(message) = serde_json::from_slice::<ArtifactMessage>(line)
            && message.package_id.starts_with(BEVY_PACKAGE_ID_PREFIX)
            && message.target.name == "bevy"
            && message.target.kind == ["lib"]
        {
            messages.push(message);
        }
    }

    ensure!(messages.len() == 1);

    let rlib = messages[0]
        .filenames
        .iter()
        .find(|p| p.extension() == Some(OsStr::new("rlib")))
        .unwrap();

    Ok(rlib.to_path_buf())
}
