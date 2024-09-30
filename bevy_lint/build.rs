use toml_edit::DocumentMut;

const RUST_TOOLCHAIN: &str = include_str!("../rust-toolchain.toml");

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=../rust-toolchain.toml");

    let rust_toolchain = RUST_TOOLCHAIN
        .parse::<DocumentMut>()
        .expect("Failed to parse `rust-toolchain.toml`.");

    let channel = rust_toolchain["toolchain"]["channel"].as_str().unwrap();

    println!("cargo::rustc-env=RUST_TOOLCHAIN_CHANNEL={channel}");
}
