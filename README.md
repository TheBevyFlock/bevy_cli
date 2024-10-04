# Bevy CLI

A Bevy CLI tool.

## Nightly Rust

The Bevy CLI includes a [custom linter](bevy_lint) that integrates directly with `rustc` through [`#![feature(rustc_private)]`](https://doc.rust-lang.org/nightly/unstable-book/language-features/rustc-private.html). Because of this, building this project requires nightly Rust with the `rustc-dev` and `llvm-tools-preview` components. If you use Rustup, a pinned version will be automatically installed when you compile this project based on the contents of [`rust-toolchain.toml`](rust-toolchain.toml).

> [!WARNING]
>
> Some components may still be missing due to a [`rustup` bug](https://github.com/rust-lang/rustup/issues/3255). If you get `can't find crate` errors when trying to build, ensure that you have the toolchain and components installed, based on [`rust-toolchain.toml`](rust-toolchain.toml).
