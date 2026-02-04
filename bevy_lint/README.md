<div class = "rustdoc-hidden">

# `bevy_lint`

`bevy_lint` is a custom linter for the [Bevy game engine](https://bevy.org), similar to [Clippy](https://doc.rust-lang.org/stable/clippy).

</div>

- [**Documentation**](https://thebevyflock.github.io/bevy_cli/linter/index.html)
- [**All Lints**]
- [**Repository**](https://github.com/TheBevyFlock/bevy_cli)
- [**Issue Tracker**](https://github.com/TheBevyFlock/bevy_cli/issues?q=is%3Aopen+is%3Aissue+label%3AA-Linter)

<!--
This link gets overridden when this file is rendered by `rustdoc`. For more info on how this works,
see <https://linebender.org/blog/doc-include/>.
-->
[**All Lints**]: https://thebevyflock.github.io/bevy_cli/api/bevy_lint/lints/index.html

<div class="rustdoc-alert rustdoc-alert-warning">

> **Warning**
>
> This is an unofficial community project, hacked upon by the Bevy CLI working group until it is eventually upstreamed into the main [Bevy Engine organization](https://github.com/bevyengine). Pardon our rough edges, and please consider [submitting an issue](https://github.com/TheBevyFlock/bevy_cli/issues) if you run into trouble!

</div>

## Installation

`bevy_lint` depends on a pinned nightly version of Rust with the `rustc-dev` Rustup component. This is because `bevy_lint` uses [internal `rustc` crates](https://doc.rust-lang.org/nightly/nightly-rustc/) that can only be imported with the permanently-unstable [`rustc_private` feature](https://doc.rust-lang.org/nightly/unstable-book/language-features/rustc-private.html). You can refer to the [compatibility table](https://thebevyflock.github.io/bevy_cli/linter/compatibility.html) to see which version of the linter requires which toolchain.

You can install the toolchain required for the latest release with:

```sh
rustup toolchain install nightly-2026-01-22 \
    --component rustc-dev \
    --component llvm-tools
```

If you are installing a different version of the linter, you may need to install a different nightly toolchain as specified by the [compatibility table](https://thebevyflock.github.io/bevy_cli/linter/compatibility.html). Please be aware that you must keep this toolchain installed for `bevy_lint` to function[^keep-toolchain-installed].

[^keep-toolchain-installed]: `bevy_lint` imports internal `rustc` libraries in order to hook into the compiler process. These crates are stored in a [dynamic library](https://en.wikipedia.org/wiki/Dynamic_linker) that is installed with the `rustc-dev` component and loaded by `bevy_lint` at runtime. Uninstalling the nightly toolchain would remove this dynamic library, causing `bevy_lint` to fail.

Once you have the toolchain installed, you can compile and install `bevy_lint` through `cargo`:

```sh
rustup run nightly-2026-01-22 cargo install \
    --git https://github.com/TheBevyFlock/bevy_cli.git \
    --tag lint-v0.6.0 \
    --locked \
    bevy_lint
```

If you're installing a different version of the linter, you may need to switch the toolchain and tag in the above command.

## Getting Started

`bevy_lint` has the same API as the `cargo check` command:

```sh
bevy_lint --help
```

If you have the [prototype Bevy CLI](https://thebevyflock.github.io/bevy_cli/cli/index.html) installed, the linter is also available through the `lint` subcommand:

```sh
bevy lint --help
```

For more information on how to use the website, please see the [full documentation](https://thebevyflock.github.io/bevy_cli/linter/index.html).

## License

The Bevy Linter is licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](https://github.com/TheBevyFlock/bevy_cli/blob/main/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](https://github.com/TheBevyFlock/bevy_cli/blob/main/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Please see [`CONTRIBUTING.md`](https://github.com/TheBevyFlock/bevy_cli/blob/main/CONTRIBUTING.md) for the CLI for more information! The linter contributing guide can be found [on the website](https://thebevyflock.github.io/bevy_cli/contribute/linter/index.html).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
