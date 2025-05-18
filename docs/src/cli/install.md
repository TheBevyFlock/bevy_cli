# Installation

<!-- Please keep this section synchronized with the `README.md`. -->

As the CLI is currently an unofficial tool, it is not yet published to <https://crates.io>. It is available [on Github](https://github.com/TheBevyFlock/bevy_cli), however.

## Precompiled Binary

The CLI is precompiled for Linux, Windows, and MacOS. You may install the latest precompiled binary using [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall):

```sh
cargo binstall --git https://github.com/TheBevyFlock/bevy_cli --locked bevy_cli
```

You can manually download the precompiled binaries from the [release page](https://github.com/TheBevyFlock/bevy_cli/releases).

## Build from Source

You may compile the CLI from scratch using `cargo install`. To install the latest release, make sure to specify the version you wish in the tag (ex. `--tag cli-v0.1.0`).

```sh
cargo install --git https://github.com/TheBevyFlock/bevy_cli --tag cli-vX.Y.Z --locked bevy_cli
```

### Bleeding Edge

<div class="warning">

**Here be dragons! 🐉**

You may run into bugs when using the unstable version of the CLI. You've been warned, and have fun! :)

</div>

If you want to try out the newest unstable features, you may install the CLI from the [`main`](https://github.com/TheBevyFlock/bevy_cli/tree/main) branch:

```sh
cargo install --git https://github.com/TheBevyFlock/bevy_cli --branch main --locked bevy_cli
```
