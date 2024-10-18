# `bevy_lint`

`bevy_lint` is a custom linter for the [Bevy game engine](https://bevyengine.org), similar to [Clippy](https://doc.rust-lang.org/stable/clippy).

## Installation

`bevy_lint` uses [`#![feature(rustc_private)]`](https://doc.rust-lang.org/nightly/unstable-book/language-features/rustc-private.html) to link to `rustc` crates. As such, it requires a specific nightly toolchain to be installed.

### Bleeding edge

You can install `bevy_lint` directly from the Git repository ([TheBevyFlock/bevy_cli](https://github.com/TheBevyFlock/bevy_cli)) to try out new and unstable features!

First, you must install the toolchain and components described by [`rust-toolchain.toml`](https://github.com/TheBevyFlock/bevy_cli/blob/main/rust-toolchain.toml) using [Rustup]. As of the time of writing (October 17th, 2024), the command may look like this:

```bash
$ rustup toolchain install nightly-2024-10-03 --component rustc-dev --component llvm-tools-preview
```

Please be aware that you must keep this toolchain installed for `bevy_lint` to function[^0].

Next, install the actual linter from Git:

```bash
$ cargo +nightly-2024-10-03 install --git https://github.com/TheBevyFlock/bevy_cli.git --locked bevy_lint
```

> [!IMPORTANT]
>
> Make sure to specify the correct nightly toolchain (such as `nightly-2024-10-03`) when running `cargo install`.

[Rustup]: https://rustup.rs

[^0]: `bevy_lint` interfaces with `rustc` to setup custom lints. It does not bundle all of `rustc` into the executable, though, and instead dynamically links to `librustc_driver.so` at runtime. `librustc_driver.so` is installed with the toolchain, so removing the toolchain will cause `bevy_lint` to fail to link.

## Usage

`bevy_lint` has the same API as `cargo check`:

```bash
$ bevy_lint --help
```

If you have the Bevy CLI installed, the linter as available as the `lint` subcommand:

```bash
$ bevy lint --help
```

## License

The Bevy Linter is licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([`LICENSE-MIT`](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Please see [`CONTRIBUTING.md`](../CONTRIBUTING.md) for more information!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
