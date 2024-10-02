use toml_edit::DocumentMut;

const RUST_TOOLCHAIN: &str = include_str!("../rust-toolchain.toml");

fn main() {
    // Only re-run this build script if its source or `rust-toolchain.toml` was modified.
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=../rust-toolchain.toml");

    let rust_toolchain = RUST_TOOLCHAIN
        .parse::<DocumentMut>()
        .expect("Failed to parse `rust-toolchain.toml`.");

    let channel = rust_toolchain["toolchain"]["channel"]
        .as_str()
        .expect("Could not find `toolchain.channel` key in `rust-toolchain.toml`.");

    // Emit the toolchain channel as an environmental variable that the crate can access using the
    // `env!()` macro.
    println!("cargo::rustc-env=RUST_TOOLCHAIN_CHANNEL={channel}");
}
