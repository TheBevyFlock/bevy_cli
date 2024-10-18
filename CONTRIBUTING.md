# Contributing

Thank you for your interest in the Bevy CLI! Make sure to join the [Bevy Discord] and check out the [working group channel] for the latest information on the CLI and linter.

[Bevy Discord]: https://discord.gg/bevy
[working group channel]: https://discord.com/channels/691052431525675048/1278871953721262090

## Getting Started

Feel free to pick an issue from the [issue tracker], fork the repository, and submit a PR for it! You can also help out by participating in conversation (both on Github and Discord) and [reviewing others' pull requests].

[issue tracker]: https://github.com/TheBevyFlock/bevy_cli/issues
[reviewing others' pull requests]: https://github.com/TheBevyFlock/bevy_cli/pulls

### Nightly Rust

`bevy_lint` requires a specific nightly Rust toolchain in order to link to `rustc` crates, described in [`rust-toolchain.toml`](rust-toolchain.toml). If you have [Rustup] installed, this toolchain should be automatically installed once you run `rustc` or `cargo` within the workspace.

> [!WARNING]
>
> Some components may still be missing due to a [`rustup` bug](https://github.com/rust-lang/rustup/issues/3255). If you get `can't find crate` errors when trying to build, ensure that you have the toolchain and components installed, based on [`rust-toolchain.toml`](rust-toolchain.toml).

[Rustup]: https://rustup.rs
