# Installation

## CLI

The CLI supports automatically installing the linter. Make sure you [have the CLI first](../cli/install.md), then simply run `bevy lint install v0.6.0`.

The CLI will prompt you if you wish to install the linter and the required toolchain. Note that it will assume you are using Rustup, and if that isn't the case you should [install the linter manually instead](#manual-without-rustup).

```
Do you want to install `bevy_lint-v0.6.0` and the required toolchain: `nightly-2026-01-22` ? [y/n]
```

If you want to auto-confirm the prompt, you may pass `--yes` to the command. Note that if you are installing the linter in CI, you may wish to use the [dedicated Github Action instead](github-actions.md):

```sh
bevy lint install --yes v0.6.0
```

## Manual with Rustup

`bevy_lint` requires a specific nightly Rust toolchain with the `rustc-dev` and `llvm-tools` components. You can install the toolchain required for the latest release with:

```sh
rustup toolchain install nightly-2026-01-22 \
    --component rustc-dev \
    --component llvm-tools
```

If you are installing a different version of the linter, you may need to install a different nightly toolchain as specified by the [compatibility table](compatibility.md). Please be aware that you must keep this toolchain installed for `bevy_lint` to function[^keep-toolchain-installed].

[^keep-toolchain-installed]: `bevy_lint` imports internal `rustc` libraries in order to hook into the compiler process. These crates are stored in a [dynamic library](https://en.wikipedia.org/wiki/Dynamic_linker) that is loaded by `bevy_lint` at runtime. Uninstalling the nightly toolchain would remove this dynamic library, causing `bevy_lint` to fail.

Once you have the toolchain installed, you can compile and install `bevy_lint` through Cargo:

```sh
rustup run nightly-2026-01-22 cargo install \
    --git https://github.com/TheBevyFlock/bevy_cli.git \
    --tag lint-v0.6.0 \
    --locked \
    bevy_lint
```

If you're installing a different version of the linter, you may need to switch the toolchain and tag in the above command.

## Manual without Rustup

It is possible to use the linter without Rustup, however it requires some extra steps. First, you'll need to install the required nightly Rust toolchain (check the [compatibility table](compatibility.md)) through some other means. If you're using Nix, for example, you would use a [Rust overlay](https://nixos.wiki/wiki/Rust#Unofficial_overlays) to do this. Make sure to also install the `rustc-dev` and `llvm-tools` components for the toolchain.

Once you've installed the toolchain and components, use that toolchain's `cargo` to build the linter:

```sh
my-toolchain/bin/cargo install \
    --git https://github.com/TheBevyFlock/bevy_cli.git \
    --tag lint-v0.6.0 \
    --locked \
    bevy_lint
```

Next, you'll need to note down the absolute path the toolchain was installed at. You can easily find it by running:

```sh
my-toolchain/bin/rustc --print sysroot
```

Finally, you will need to set the [`BEVY_LINT_SYSROOT` environmental variable](environmental-variables.md#bevy_lint_sysroot) any time you run `bevy_lint`. The easiest way to do this on Unix-based systems is to set it in a startup script like `.bashrc` or `.profile`:

```sh
export BEVY_LINT_SYSROOT=/absolute/path/to/sysroot
```

## Uninstall

If you wish to uninstall the linter at any time, you may use Cargo to do so:

```sh
cargo uninstall bevy_lint
```

You may also wish to uninstall the Rustup toolchain. The following command will uninstall the toolchain required by the latest version of the linter, but you may need to specify a different toolchain from the [compatibility table](compatibility.md):

```sh
rustup toolchain uninstall 2025-05-14
```

## Upgrade

To upgrade to a newer version of the linter, first [uninstall it](#uninstall) then follow the [CLI](#cli) or [manual](#manual-with-rustup) installation instructions.
