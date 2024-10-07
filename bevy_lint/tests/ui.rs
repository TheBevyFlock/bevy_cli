// A convenience feature used in `find_bevy_rlib()`.
#![feature(let_chains)]

use serde::Deserialize;
use std::{
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
    const DRIVER_PATH: &str = "../target/debug/bevy_lint_driver";

    ensure!(Path::new(DRIVER_PATH).is_file());

    let bevy_extern_argument = match find_bevy_rlib() {
        Ok(path) => format!("--extern=bevy={}", path.display()).into(),
        Err(error) => {
            eprintln!("Error while finding path to `libbevy.rlib`: {:?}", error);
            "--extern=bevy".into()
        }
    };

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
                DRIVER_PATH.into(),
                // `bevy_lint_driver` expects the first argument to be the path to `rustc`.
                "rustc".into(),
                // This is required so that `ui_test` can parse warnings and errors.
                "--error-format=json".into(),
                // These two lines tell `rustc` to search in `target/debug/deps` for dependencies.
                // This is required for UI tests to import `bevy`.
                "-L".into(),
                "all=../target/debug/deps".into(),
                // This lets UI tests write `use bevy::*;` without `extern bevy;` first.
                bevy_extern_argument,
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

#[derive(Deserialize, Debug)]
#[serde(rename = "compiler-artifact", tag = "reason")]
struct ArtifactMessage<'a> {
    package_id: &'a str,
    target: ArtifactTarget<'a>,
    filenames: Vec<&'a Path>,
}

#[derive(Deserialize, Debug)]
struct ArtifactTarget<'a> {
    name: &'a str,
    kind: Vec<&'a str>,
}

fn find_bevy_rlib() -> color_eyre::Result<PathBuf> {
    let output = Command::new("cargo")
        .arg("build")
        .arg("--test=ui")
        .arg("--message-format=json")
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
        .filter(|p| p.extension() == Some(OsStr::new("rlib")))
        .next()
        .unwrap();

    Ok(rlib.to_path_buf())
}
