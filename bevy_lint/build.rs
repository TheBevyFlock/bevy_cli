//! This build script hardcodes the path to the current Rust toolchain's libraries into
//! `bevy_lint_driver` so that it can be called directly, instead of just through `cargo`. There
//! are a few limitations with this solution:
//! 
//! 1. The hardcoded path is not portable, since it references the name of the user's home
//!    directory. (E.g. it hardcodes `/Users/appleseedj/...` into the produced binary.)
//! 2. The produced binary still requires the pinned nightly toolchain to be installed.
//! 
//! For these reasons, consider this more of a hack than an all-encompassing solution.

use std::{
    path::{Path, PathBuf},
    process::Command,
    str,
};

const DRIVER: &str = "bevy_lint_driver";

fn main() {
    // Only re-run build script if it was modified. If it wasn't, the cached output is instead used.
    println!("cargo::rerun-if-changed=build.rs");

    // Find the folder where the `librustc_driver` dynamic library is stored.
    let library_path = locate_toolchain_libraries();

    // Instruct the linker to hardcode the library path into `bevy_lint_driver`.
    //
    // `-Wl` instructs `cc` to pass the following comma-separated arguments to the linker, `ld` in
    // this case. `-rpath` instructs the linker to hardcode the path into the binary, so that the
    // dynamic linker will search there for `librustc_driver`. See <https://en.wikipedia.org/wiki/Rpath>
    // for further details.
    println!(
        "cargo::rustc-link-arg-bin={DRIVER}=-Wl,-rpath,{}",
        library_path.display()
    );
}

/// Finds the path to `~/.rustup/toolchains/*/lib` of the current toolchain using `rustup`.
fn locate_toolchain_libraries() -> PathBuf {
    let rustup_output = Command::new("rustup")
        .arg("which")
        .arg("rustc")
        .output()
        .expect("Failed to locate `rustc` using `rustup`.");

    assert!(
        rustup_output.status.success(),
        "Calling `rustup which rustc` failed with non-zero exit code."
    );

    // We're assuming the path will always be valid UTF-8. If this throws an error for you, please
    // report an issue and we'll fix it!
    let rustc_path = Path::new(
        str::from_utf8(&rustup_output.stdout).expect("`rustup` did not emit valid UTF-8."),
    );

    // From `~/.rustup/toolchains/*/bin/rustc`, find `~/.rustup/toolchains/*/lib`.
    let library_path = rustc_path
        .parent()
        .and_then(|p| p.parent())
        .expect(&format!(
            "Failed to find toolchain path from `rustc` at {}",
            rustc_path.display()
        ))
        .join("lib");

    assert!(
        library_path.exists(),
        "Toolchain library path does not exist at {}.",
        library_path.display()
    );

    library_path
}
