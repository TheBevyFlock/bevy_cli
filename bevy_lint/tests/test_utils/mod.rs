use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use serde::Deserialize;
use ui_test::{
    CommandBuilder, Config,
    color_eyre::{
        self,
        eyre::{bail, ensure},
    },
};

// This is set by Cargo to the absolute path of `bevy_lint_driver`.
const DRIVER_PATH: &str = env!("CARGO_BIN_EXE_bevy_lint_driver");

/// Generates a custom [`Config`] for `bevy_lint`'s UI tests.
pub fn base_config(test_dir: &str) -> color_eyre::Result<Config> {
    let driver_path = Path::new(DRIVER_PATH);

    ensure!(
        driver_path.is_file(),
        "`bevy_lint_driver` could not be found at {}, make sure to build it with `cargo build -p bevy_lint --bin bevy_lint_driver`",
        driver_path.display(),
    );

    let config = Config {
        // When `host` is `None`, `ui_test` will attempt to auto-discover the host by calling
        // `program -vV`. Unfortunately, `bevy_lint_driver` does not yet support the version flag,
        // so we manually specify the host as an empty string. This means that, for now, host-
        // specific configuration in UI tests will not work.
        host: Some(String::new()),
        program: CommandBuilder {
            // We don't need `rustup run` here because we're already using the correct toolchain
            // due to `rust-toolchain.toml`.
            program: driver_path.into(),
            args: vec![
                // `bevy_lint_driver` expects the first argument to be the path to `rustc`.
                "rustc".into(),
                // This is required so that `ui_test` can parse warnings and errors.
                "--error-format=json".into(),
                // These two lines tell `rustc` to search in `target/debug/deps` for dependencies.
                // This is required for UI tests to import `bevy`.
                "-L".into(),
                "all=..\\target\\debug\\deps".into(),
                // Make the `bevy` crate directly importable from the UI tests.
                format!("--extern=bevy={}", find_bevy_rlib()?.display()).into(),
            ],
            out_dir_flag: Some("--out-dir".into()),
            input_file_flag: None,
            envs: Vec::new(),
            cfg_flag: Some("--print=cfg".into()),
        },
        out_dir: PathBuf::from("../target/ui"),
        ..Config::rustc(Path::new("tests").join(test_dir))
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
struct ArtifactMessage {
    target: ArtifactTarget,
    filenames: Vec<PathBuf>,
}

/// The `"target"` field of an [`ArtifactMessage`].
#[derive(Deserialize, Debug)]
struct ArtifactTarget {
    name: String,
    kind: Vec<String>,
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

    ensure!(output.status.success(), "`cargo build --test=ui` failed.");

    // It's theoretically possible for there to be multiple messages about building `libbevy.rlib`.
    // We support this, but optimize for just 1 message.
    let mut messages = Vec::with_capacity(1);

    // Convert the `stdout` to a string, replacing invalid characters with `�`.
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Iterate over each line in stdout, trying to deserialize it from JSON.
    for line in stdout.lines() {
        if let Ok(message) = serde_json::from_str::<ArtifactMessage>(line)
            // If the message passes the following conditions, it's probably the one we want.
            && message.target.name == "bevy"
            && message.target.kind.iter().any(|s| s == "lib")
        {
            messages.push(message);
        }
    }

    match messages.len() {
        // Usually there should only be one message that `bevy` was built, in which case we
        // continue the program.
        1 => {}

        // Both of these are failure cases where we exit early.
        0 => bail!("`bevy` was not built, but is required for ui tests"),
        len @ 2.. => {
            bail!("`bevy` was built {len} times, but it was only expected to be built once")
        }
    }

    // The message usually has multiple files, often `libbevy.rlib` and `libbevy.rmeta`. Filter
    // through these to find the `rlib`.
    let rlib = messages[0]
        .filenames
        .iter()
        .find(|p| p.extension() == Some(OsStr::new("rlib")))
        .expect("`libbevy.rlib` not found within artifact message filenames.");

    Ok(rlib.to_path_buf())
}
