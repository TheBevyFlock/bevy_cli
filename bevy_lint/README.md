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

<div class="rustdoc-alert rustdoc-alert-note">

> **Note**
>
> `bevy_lint` checks your code with the nightly toolchain it was installed with, meaning you _do_ have access to unstable features when it is called. This is best used when [detecting `bevy_lint`](#detecting-bevy_lint).

</div>

### Detecting `bevy_lint`

The linter passes `--cfg bevy_lint` when it checks your code, allowing you to detect it:

```rust,ignore
// Conditionally include this function only when `bevy_lint` is used.
#[cfg(bevy_lint)]
fn foo() {
    // ...
}

// Conditionally add an attribute only when `bevy_lint` is used.
#[cfg_attr(bevy_lint, ...)]
struct Foo;
```

If you use this, you may also need to register `bevy_lint` as a valid `cfg` flag in your `Cargo.toml`:

```toml
[lints.rust]
unexpected_cfg = { level = "warn", check-cfg = ["cfg(bevy_lint)"] }
```

### Configuring Lints

If you wish to enable and disable certain lints, you must register `bevy` as a tool. Not doing so will cause `#[allow(bevy::lint_name)]` and related attributes to fail to compile.

You can register a new tool using the `#![register_tool(...)]` attribute, which is [currently unstable](https://doc.rust-lang.org/nightly/unstable-book/language-features/register-tool.html). This isn't an issue if you [detect when `bevy_lint` is enabled](#detecting-bevy_lint), since it is guaranteed to check your code using nightly Rust.

```rust,ignore
// When `bevy_lint` is used, enable the `register_tool` feature and register the `bevy` namespace
// as a tool.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
```

You can now toggle lints throughout your project, as long as they too are behind `#[cfg_attr(bevy_lint, ...)]`:

```rust,ignore
// Enable pedantic lints, which are off by default.
#![cfg_attr(bevy_lint, warn(bevy::pedantic))]

// Deny methods of `World` in this system that can panic when a non-panicking alternative exists.
#[cfg_attr(bevy_lint, deny(bevy::panicking_world_methods))]
fn my_critical_system(world: &mut World) {
    // ...
}
```

There are several other ways to configure lints, but they have varying levels of support:

|Lint Configuration|Support|Additional Information|
|-|-|-|
|`#[allow(...)]` and related|✅|Must be behind `#[cfg_attr(bevy_lint, ...)]` on stable Rust.|
|`[lints.bevy]` in `Cargo.toml`|⚠️|Nightly only because `#[register_tool(bevy)]` must always be enabled.|
|`[workspace.lints.bevy]`|❌|No current method to register `bevy` as a tool on a workspace level.|
|`RUSTFLAGS="-A bevy::lint"`|❌|`RUSTFLAGS` applies to dependencies, but they do not have `#[register_tool(bevy)]`.|

<div class="rustdoc-alert rustdoc-alert-tip">

> **Tip**
>
> If your project already uses nightly Rust, you can forego the `#[cfg_attr(bevy_lint, ...)]` attributes and write `#![feature(register_tool)]` and `#![register_tool(bevy)]` directly. This will let you configure lints using the `[lints.bevy]` table in `Cargo.toml`.

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
