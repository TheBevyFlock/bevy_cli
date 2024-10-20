# Setup

## Cloning the Repository

The first step to contributing is to [fork](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/fork-a-repo) the [`bevy_cli` repository](https://github.com/TheBevyFlock/bevy_cli). Because most people do not have write permissions to the source repository, they need to fork it to create a copy that they can push commits to. You can still open a pull request from a fork, though, so don't worry!

After you've forked the repository, clone your fork onto your computer:

```bash
$ git clone https://github.com/YOUR_USERNAME/bevy_cli.git
```

## Opening the Project

Next, open up the newly-created `bevy_cli` folder with your favorite editor. All linter-related code is within the `bevy_lint` folder.

> [!TIP]
>
> Flavors of VSCode should work out-of-the-box, but you may have some issues with other editors like RustRover, Fleet, or Emacs. This may find [this page](https://model-checking.github.io/kani/rustc-hacks.html) useful for troubleshooting issues.

## Installing the Toolchain

`bevy_lint` requires a pinned nightly toolchain in order to depend on internal `rustc` crates. When you first run `rustc` or `cargo` within this project, Rustup should automatically install this toolchain based on [`rust-toolchain.toml`](../../rust-toolchain.toml).

> [!WARNING]
>
> Some components may still be missing due to a [`rustup` bug](https://github.com/rust-lang/rustup/issues/3255). If you get `can't find crate` errors when trying to build, ensure that you have the `rustc-dev` component installed for the specific toolchain.

## Building the Project

In order to use `bevy_lint`, two executables need to be created: `bevy_lint` and `bevy_lint_driver`.

|Executable|Purpose|
|-|-|
|`bevy_lint`|Act as `cargo check`.|
|`bevy_lint_driver`|Act as `rustc`.[^0]|

[^0]: `bevy_lint_driver` requires that the first argument passed to it is the path to the actual `rustc` binary. Calling `bevy_lint_driver main.rs` will not actually compile `main.rs`, you need to call `bevy_lint_driver path/to/rustc main.rs` instead.

`bevy_lint` internally calls `bevy_lint_driver` (just as `cargo check` internally calls `rustc`). As such, calling plain `cargo run -- --help` will fail because _just_ `bevy_lint` will be built, not `bevy_lint_driver`. Instead, you need to do the following:

```bash
$ cargo build && cargo run -- --help
# Shortened:
$ cargo b && cargo r -- --help
```

This builds both binaries and then executes `bevy_lint`, which should then print a help screen!
