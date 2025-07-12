use serde::Deserialize;

const RUST_TOOLCHAIN: &str = include_str!("../rust-toolchain.toml");

/// Represents the contents of `rust-toolchain.toml`.
#[derive(Deserialize)]
struct RustToolchain {
    toolchain: Toolchain,
}

#[derive(Deserialize)]
struct Toolchain {
    channel: String,
}

fn main() {
    // Only re-run this build script if its source or `rust-toolchain.toml` was modified.
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=../rust-toolchain.toml");

    let rust_toolchain: RustToolchain =
        toml::from_str(RUST_TOOLCHAIN).expect("could not deserialize `rust-toolchain.toml`");

    let channel = rust_toolchain.toolchain.channel;

    // Emit the toolchain channel as an environmental variable that the crate can access using the
    // `env!()` macro.
    println!("cargo::rustc-env=RUST_TOOLCHAIN_CHANNEL={channel}");
}
