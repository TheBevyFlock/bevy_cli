use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use assert_cmd::cargo::cargo_bin_cmd;
use serial_test::serial;

/// The path to the test repository.
fn test_path() -> PathBuf {
    PathBuf::from(format!(
        "{}/tests/bevy_cli_test",
        env!("CARGO_MANIFEST_DIR")
    ))
    .clone()
}

/// The path to the target directory of the test repository.
fn target_path() -> PathBuf {
    test_path().join("target")
}

/// Delete the artifacts in the given path.
///
/// Still retains some cached compilations to speed up compile times,
/// but ensures that binaries are created freshly.
fn clean_target_artifacts<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    if fs::exists(&path)? {
        for entry in (fs::read_dir(path)?).flatten() {
            if entry.file_type()?.is_file() {
                fs::remove_file(entry.path())?;
            }
        }
    }
    Ok(())
}

fn ensure_path_exists<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    anyhow::ensure!(
        fs::exists(path.as_ref())?,
        "artifact {} does not exist",
        path.as_ref().display()
    );
    Ok(())
}

fn executable(binary: &str) -> String {
    if cfg!(windows) {
        format!("{binary}.exe")
    } else {
        binary.to_owned()
    }
}

#[test]
#[serial]
fn should_build_native_dev() -> anyhow::Result<()> {
    let target_artifact_path = target_path().join("debug");
    clean_target_artifacts(&target_artifact_path)?;

    let mut cmd = cargo_bin_cmd!("bevy");
    cmd.current_dir(test_path())
        .args(["build", "-p=bevy_default", "--yes"]);

    cmd.assert().success();

    ensure_path_exists(target_artifact_path.join(executable("bevy_default")))
        .context("binary does not exist")
}

#[test]
#[serial]
fn should_build_native_release() -> anyhow::Result<()> {
    let target_artifact_path = target_path().join("release");
    clean_target_artifacts(&target_artifact_path)?;

    let mut cmd = cargo_bin_cmd!("bevy");
    cmd.current_dir(test_path())
        .args(["build", "-p=bevy_default", "--yes", "--release"]);
    cmd.assert().success();

    ensure_path_exists(target_artifact_path.join(executable("bevy_default")))
        .context("binary does not exist")
}

#[test]
#[serial]
fn should_build_web_dev() -> anyhow::Result<()> {
    let target_artifact_path = target_path().join("wasm32-unknown-unknown").join("web");
    clean_target_artifacts(&target_artifact_path)?;

    let mut cmd = cargo_bin_cmd!("bevy");
    cmd.current_dir(test_path())
        .args(["build", "-p=bevy_default", "--yes", "web"]);

    cmd.assert().success();

    ensure_path_exists(target_artifact_path.join("bevy_default.wasm"))
        .context("Wasm executable does not exist")?;
    ensure_path_exists(target_artifact_path.join("bevy_default_bg.wasm"))
        .context("Wasm bindings do not exist")?;
    ensure_path_exists(target_artifact_path.join("bevy_default.js"))
        .context("JS bindings do not exist")
}

#[test]
#[serial]
fn should_build_web_release() -> anyhow::Result<()> {
    let target_artifact_path = target_path()
        .join("wasm32-unknown-unknown")
        .join("web-release");
    clean_target_artifacts(&target_artifact_path)?;

    let mut cmd = cargo_bin_cmd!("bevy");
    cmd.current_dir(test_path())
        .args(["build", "-p=bevy_default", "--release", "--yes", "web"]);

    cmd.assert().success();

    ensure_path_exists(target_artifact_path.join("bevy_default.wasm"))
        .context("Wasm executable does not exist")?;
    ensure_path_exists(target_artifact_path.join("bevy_default_bg.wasm"))
        .context("Wasm bindings do not exist")?;
    ensure_path_exists(target_artifact_path.join("bevy_default.js"))
        .context("JS bindings do not exist")
}

#[test]
#[serial]
fn should_copy_web_bundle() -> anyhow::Result<()> {
    let target_artifact_path = target_path()
        .join("wasm32-unknown-unknown")
        .join("web-release");
    clean_target_artifacts(&target_artifact_path)?;

    let _ = fs::remove_dir_all(test_path().join("web-dir"));
    let mut cmd = Command::cargo_bin("bevy")?;
    cmd.current_dir(test_path()).args([
        "build",
        "-p=bevy_default",
        "--release",
        "--yes",
        "web",
        "--bundle",
        "--bundle-dir=web-dir",
    ]);

    cmd.assert().success();

    ensure_path_exists(test_path().join("web-dir/index.html"))
        .context("index.html do not exist")?;
    ensure_path_exists(test_path().join("web-dir/build/bevy_default_bg.wasm"))
        .context("Wasm bindings do not exist")?;
    ensure_path_exists(test_path().join("web-dir/build/bevy_default.js"))
        .context("JS bindings do not exist")
}
