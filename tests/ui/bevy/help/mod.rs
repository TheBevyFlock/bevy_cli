use cargo_test_support::{cargo_test, file, str};

use crate::prelude::bevy_exe;

#[cargo_test]
fn foo() {
    snapbox::cmd::Command::new(bevy_exe())
        .arg("--help")
        .assert()
        .success()
        .stdout_eq(file!["stdout.term.svg"])
        .stderr_eq(str![]);
}
