# Installation

## CLI

The CLI supports automatically installing the latest released version of the linter if you do not have it installed already. Make sure you [have the CLI first](../cli/install.md), then simply run the `lint` subcommand:

```sh
bevy lint
```

The CLI will prompt you if you wish to install the linter. Type `y` and press enter to accept:

```
warning: failed to run bevy_lint, trying to find automatic fix...
`bevy_lint` is missing, should I install it for you? [y/n]
```

If you want to auto-confirm the prompt, you may pass `--yes` to the command. Note that if you are installing the linter in CI, you may wish to use the [dedicated Github Action instead](github-actions.md):

```sh
bevy lint --yes
```

## Manual

`bevy_lint` depends on a pinned nightly version of Rust with the `rustc-dev` Rustup component. This is because `bevy_lint` uses [internal `rustc` crates](https://doc.rust-lang.org/nightly/nightly-rustc/) that can only be imported with the permanently-unstable [`rustc_private` feature](https://doc.rust-lang.org/nightly/unstable-book/language-features/rustc-private.html). You can refer to the [compatibility table](compatibility.md) to see which version of the linter requires which toolchain.

You can install the toolchain required for the latest release with:

```sh
rustup toolchain install nightly-2025-04-03 \
    --component rustc-dev \
    --component llvm-tools-preview
```

If you are installing a different version of the linter, you may need to install a different nightly toolchain as specified by the [compatibility table](compatibility.md). Please be aware that you must keep this toolchain installed for `bevy_lint` to function[^keep-toolchain-installed].

[^keep-toolchain-installed]: `bevy_lint` imports internal `rustc` libraries in order to hook into the compiler process. These crates are stored in a [dynamic library](https://en.wikipedia.org/wiki/Dynamic_linker) that is installed with the `rustc-dev` component and loaded by `bevy_lint` at runtime. Uninstalling the nightly toolchain would remove this dynamic library, causing `bevy_lint` to fail.

Once you have the toolchain installed, you can compile and install `bevy_lint` through `cargo`:

```sh
rustup run nightly-2025-04-03 cargo install \
    --git https://github.com/TheBevyFlock/bevy_cli.git \
    --tag lint-v0.3.0 \
    --locked \
    bevy_lint
```

If you're installing a different version of the linter, you may need to switch the toolchain and tag in the above command.

## Uninstall

If you wish to uninstall the linter at any time, you may use Cargo and Rustup to do so:

```sh
cargo uninstall bevy_lint
rustup toolchain uninstall nightly-2025-04-03
```

If you're uninstalling an older version of the linter, such as when you are [upgrading](#upgrade), you may need to uninstall a different toolchain version than the one in the above command. Check out the [compatibility table](compatibility.md) to see which version uses which toolchain.

## Upgrade

To upgrade to a newer version of the linter, first [uninstall it](#uninstall) then follow the [CLI](#cli) or [manual](#manual) installation instructions.
