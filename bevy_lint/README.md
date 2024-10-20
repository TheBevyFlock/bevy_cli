<div class = "rustdoc-hidden">

# `bevy_lint`

`bevy_lint` is a custom linter for the [Bevy game engine](https://bevyengine.org), similar to [Clippy](https://doc.rust-lang.org/stable/clippy).

</div>

<div class="rustdoc-alert rustdoc-alert-warning">

> **Warning**
>
> This is an unofficial community project, hacked upon by the Bevy CLI working group until it is eventually upstreamed into the main [Bevy Engine organization]. Pardon our rough edges!

</div>

[Bevy Engine organization]: https://github.com/bevyengine

## Installation

`bevy_lint` uses [`#![feature(rustc_private)]`](https://doc.rust-lang.org/nightly/unstable-book/language-features/rustc-private.html) to link to `rustc` crates. As such, it requires a specific nightly toolchain to be installed.

### Bleeding edge

You can install `bevy_lint` directly from the Git repository ([TheBevyFlock/bevy_cli](https://github.com/TheBevyFlock/bevy_cli)) to try out new and unstable features!

First, you must install the toolchain and components described by [`rust-toolchain.toml`](https://github.com/TheBevyFlock/bevy_cli/blob/main/rust-toolchain.toml) using [Rustup]. As of the time of writing (October 17th, 2024), the command may look like this:

```bash
$ rustup toolchain install nightly-2024-10-03 \
      --component rustc-dev \
      --component llvm-tools-preview
```

Please be aware that you must keep this toolchain installed for `bevy_lint` to function[^0].

Next, install the actual linter from Git:

```bash
$ cargo +nightly-2024-10-03 install \
      --git https://github.com/TheBevyFlock/bevy_cli.git \
      --locked \
      bevy_lint
```

<div class="rustdoc-alert rustdoc-alert-important">

> **Important**
>
> Make sure to specify the correct nightly toolchain (such as `nightly-2024-10-03`) when running `cargo install`.

</div>

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

### Configuring Lints

Toggling lints and lint groups requires the nightly `register_tool` feature. Even if your project uses stable Rust, you can still use this feature by detecting the `--cfg bevy_lint` flag:

```rust,ignore
// When `--cfg bevy_lint` is passed, enable the nightly `register_tool` feature and register
// `bevy`.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
```

`bevy_lint` checks your project with a nightly toolchain and automatically passes `--cfg bevy_lint`. Make sure to add it to the list of known `--cfg` flags in `Cargo.toml`:

```toml
[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ["cfg(bevy_lint)"] }
```

You can now toggle lints and lint groups throughout the crate, as long as they are also behind `#[cfg_attr(...)]`:

```rust,ignore
#![cfg_attr(bevy_lint, warn(bevy::pedantic))]

#[cfg_attr(bevy_lint, deny(bevy::panicking_world_methods))]
fn my_critical_system(world: &mut World) {
    // ...
}
```

If you do not register `bevy` as a tool, `#[allow(bevy::lint_name)]` and related attributes will fail to compile.

|Lint Configuration|Support|Additional Information|
|-|-|-|
|`#[allow(...)]` and related|✅|Must be behind `#[cfg_attr(bevy_lint, ...)]` on stable Rust|
|`[lints.bevy]` in `Cargo.toml`|⚠️|(Nightly only because `#[register_tool(bevy)]` must not be behind `#[cfg_attr(bevy_lint, ...)]`)|
|`RUSTFLAGS="-A bevy::lint"`|❌|`RUSTFLAGS` applies to dependencies, but they do not `#[register_tool(bevy)]`|

<div class="rustdoc-alert rustdoc-alert-tip">

> **Tip**
>
> If your project uses nightly Rust by default, you can forego the `#![cfg_attr(...)]` and write `#![feature(register_tool)]` and `#![register_tool(bevy)]` directly. This will let you configure lints using the `[lints.bevy]` table in `Cargo.toml`.

</div>

## Compatibility

|`bevy_lint` Version|Rust Version|Rustup Toolchain|Bevy Version|
|-|-|-|-|
|0.1.0-dev|1.83.0|`nightly-2024-10-03`|0.14|

## License

The Bevy Linter is licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](https://github.com/TheBevyFlock/bevy_cli/blob/main/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](https://github.com/TheBevyFlock/bevy_cli/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Please see [`CONTRIBUTING.md`](https://github.com/TheBevyFlock/bevy_cli/blob/main/CONTRIBUTING.md) for more information!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
