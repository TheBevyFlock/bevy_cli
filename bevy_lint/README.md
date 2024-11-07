<div class = "rustdoc-hidden">

# `bevy_lint`

`bevy_lint` is a custom linter for the [Bevy game engine](https://bevyengine.org), similar to [Clippy](https://doc.rust-lang.org/stable/clippy).

</div>

<div class="rustdoc-alert rustdoc-alert-warning">

> **Warning**
>
> This is an unofficial community project, hacked upon by the Bevy CLI working group until it is eventually upstreamed into the main [Bevy Engine organization](https://github.com/bevyengine). Pardon our rough edges!

</div>

## Installation

`bevy_lint` depends on a pinned nightly version of Rust with the `rustc-dev` Rustup component. You can refer to the [compatibility table](#compatibility) to see which version of the linter requires which toolchain.

You can install the toolchain with:

```bash
rustup toolchain install $TOOLCHAIN_VERSION \
    --component rustc-dev \
    --component llvm-tools-preview
```

For example, you would replace `$TOOLCHAIN_VERSION` with `nightly-2024-10-03` if you were installing `bevy_lint` 0.1.0. Please be aware that you must keep this toolchain installed for `bevy_lint` to function[^keep-toolchain-installed].

[^keep-toolchain-installed]: `bevy_lint` imports internal `rustc` libraries in order to hook into the compiler process. These crates are stored in a [dynamic library](https://en.wikipedia.org/wiki/Dynamic_linker) that is installed with the `rustc-dev` component and loaded by `bevy_lint` at runtime. Uninstalling the nightly toolchain would remove this dynamic library, causing `bevy_lint` to fail.

Once you have the toolchain installed, you can compile and install `bevy_lint` through `cargo`:

```bash
rustup run $TOOLCHAIN_VERSION cargo install \    
    --git https://github.com/TheBevyFlock/bevy_cli.git \
    --tag $TAG \
    --locked \
    bevy_lint
```

Make sure to replace `$TOOLCHAIN_VERSION` and `$TAG` in the above command. The tag for a specific release can be found in the [releases tab](https://github.com/TheBevyFlock/bevy_cli/releases). For example, the tag for 0.1.0 is `lint-v0.1.0`.

## Usage

`bevy_lint` has the same API as the `cargo check` command:

```bash
bevy_lint --help
```

If you have the Bevy CLI installed, the linter is also available through the `lint` subcommand:

```bash
bevy lint --help
```

The linter passes `--cfg bevy_lint` when it checks your code, allowing you to detect it:

```rust
#[cfg(bevy_lint)]
```

If you use this, you may also need to register `bevy_lint` as a valid `cfg` flag in your `Cargo.toml`:

```toml
[lints.rust]
unexpected_cfg = { level = "warn", check-cfg = ["cfg(bevy_lint)"] }
```

## Compatibility

|`bevy_lint` Version|Rust Version|Rustup Toolchain|Bevy Version|
|-|-|-|-|
|0.1.0-dev|1.83.0|`nightly-2024-10-03`|0.14|

## License

The Bevy Linter is licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](https://github.com/TheBevyFlock/bevy_cli/blob/main/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](https://github.com/TheBevyFlock/bevy_cli/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
