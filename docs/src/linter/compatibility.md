# Compatibility

|`bevy_lint` Version|Rust Version|Rustup Toolchain|Bevy Version|
|-|-|-|-|
|0.7.0-dev|1.95.0|`nightly-2026-01-22`|0.18|
|0.6.0|1.95.0|`nightly-2026-01-22`|0.18|
|0.5.0|1.94.0|`nightly-2025-12-11`|0.17|
|0.4.0|1.90.0|`nightly-2025-06-26`|0.16|
|0.3.0|1.88.0|`nightly-2025-04-03`|0.16|
|0.2.0|1.87.0|`nightly-2025-02-20`|0.15|
|0.1.0|1.84.0|`nightly-2024-11-14`|0.14|

The Rust version in the above table specifies what [version of the Rust language](https://github.com/rust-lang/rust/releases) can be compiled with `bevy_lint`. Code written for a later version of Rust may not compile. (This is not usually an issue, though, because `bevy_lint`'s Rust version is kept 1 to 2 releases ahead of stable Rust.)

The Rustup toolchain specifies which toolchain must be installed in order for `bevy_lint` to be installed and used. Please see [the installation section](install.md) for more info.

The Bevy version is a range of Bevy versions that `bevy_lint` has been tested with and is guaranteed to work. Newer or older releases may not be linted correctly and may cause the linter to crash. (If this does happen for you, please consider [submitting a bug report](https://github.com/TheBevyFlock/bevy_cli/issues)!)
