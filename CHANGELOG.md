# Changelog

All notable user-facing changes to the **Bevy CLI** will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## Unreleased

**All Changes**: [`cli-v0.1.0-alpha.1...main`](https://github.com/TheBevyFlock/bevy_cli/compare/cli-v0.1.0-alpha.1...main)

### Added

- The Bevy CLI now has colorful `--help` output, matching Cargo's style ([#464](https://github.com/TheBevyFlock/bevy_cli/pull/464))
- The main `--help` message now contains links to Bevy's website, Bevy's source code, and the CLI's documentation ([#457](https://github.com/TheBevyFlock/bevy_cli/pull/457), [#482](https://github.com/TheBevyFlock/bevy_cli/pull/482))
- You can now specify the host address in `bevy run web` with the `--host` flag ([#472](https://github.com/TheBevyFlock/bevy_cli/pull/472))
- It is now possible to customize the arguments passed to `wasm-opt` when optimizing a build for the web ([#486](https://github.com/TheBevyFlock/bevy_cli/pull/486))
  - You can pass flags to the CLI like `--wasm-opt=-Oz --wasm-opt=--enable-bulk-memory`, or you can change the `wasm-opt` configuration value to an array instead of a boolean, like `wasm-opt = ["-Oz", "--enable-bulk-memory"]`
- You can now run the linter on web builds with `bevy lint web` ([#523](https://github.com/TheBevyFlock/bevy_cli/pull/523))
- It is now possible to specify `bevy run web` HTTP headers in `Cargo.toml` using the `headers` array ([#544](https://github.com/TheBevyFlock/bevy_cli/pull/544))
- (Unstable) It is now possible to build apps for the web with support for multi-threading ([#499](https://github.com/TheBevyFlock/bevy_cli/pull/499))
  - Use the `--unstable multi-threading` flag or set the `unstable.web-multi-threading` config to `true` to run an app using multi-threaded Wasm
  - This is only available when compiling the CLI with the `unstable` feature, however it is enabled by default
  - This depends on an unstable flag in the Rust compiler, and thus requires a nightly toolchain of Rust
  - Bevy doesn't natively utilize multi-threaded WASM, so this will only benefit plugins that support it or code you write yourself
- The CLI is now able to list and install arbitrary versions of the Bevy linter with `bevy lint list` and `bevy lint install` ([#529](https://github.com/TheBevyFlock/bevy_cli/pull/529))
- `bevy build web` and `bevy run web` commands will now automatically apply the web backend for `getrandom` if necessary. `getrandom` requires both a feature and a rustflag to be enabled by the user, which can quickly lead to compile errors when not set up correctly. The CLI will automatically set the rustflag if needed and provide easy instructions on how to configure the features ([#547](https://github.com/TheBevyFlock/bevy_cli/pull/547))

### Changed

- The `--no-default-features` option confusingly used to accept `true` and `false ` as inputs, but has been changed to be a simple toggle flag, matching Cargo's behavior ([#473](https://github.com/TheBevyFlock/bevy_cli/pull/473))
- When building a project, the CLI can now load the `assets` and `web` folders next to the crate `Cargo.toml`, rather than just the workspace `Cargo.toml`. Crate-specific folders will be prioritized over workspace folders ([#485](https://github.com/TheBevyFlock/bevy_cli/pull/485), [#490](https://github.com/TheBevyFlock/bevy_cli/pull/490))
- `bevy build web` and `bevy run web` now support [JS snippets](https://wasm-bindgen.github.io/wasm-bindgen/reference/js-snippets.html) ([#527](https://github.com/TheBevyFlock/bevy_cli/pull/527))
- The error message has been improved in cases where the CLI fails to gather project information using `cargo metadata` ([#543](https://github.com/TheBevyFlock/bevy_cli/pull/543), [#545](https://github.com/TheBevyFlock/bevy_cli/pull/545))
- `rustflags` specified in [`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html#configuration) and the `RUSTFLAGS` environmental variable are now respected and will be merged with those defined in the CLI's configuration ([#540](https://github.com/TheBevyFlock/bevy_cli/pull/540))
- The `--yes` flag from `bevy lint` has been moved to the `bevy lint install` subcommand ([#583](https://github.com/TheBevyFlock/bevy_cli/pull/583))
- The CLI no longer uses `cargo-generate` as a library dependency. Instead, if you don't have the `cargo-generate` executable installed, the CLI will ask to automatically install it for you when you first run `bevy new` ([#597](https://github.com/TheBevyFlock/bevy_cli/pull/597))

### Fixed

- Instructions in `bevy lint --help` incorrectly said `bevy_lint` needed to be installed manually ([#465](https://github.com/TheBevyFlock/bevy_cli/pull/465))
- The example for `bevy completions` has been reworked to be clearer ([#466](https://github.com/TheBevyFlock/bevy_cli/pull/466))
- `bevy run` now respects the `default-members` workspace field in `Cargo.toml` when selecting which crate to run ([#477](https://github.com/TheBevyFlock/bevy_cli/pull/477))
- `bevy new` now emits an error if the name of the crate is invalid ([#480](https://github.com/TheBevyFlock/bevy_cli/pull/480))
- `bevy build web` and `bevy run web` now respect the `target` configuration option, meaning it's now possible to build for WASM targets like `wasm32v1-none` instead of just `wasm32-unknown-unknown` ([#481](https://github.com/TheBevyFlock/bevy_cli/pull/481))

## v0.1.0-alpha.1 - 2025-05-23

**All Changes**: [`cli-v0.1.0-alpha.1`](https://github.com/TheBevyFlock/bevy_cli/commits/cli-v0.1.0-alpha.1)

### Added

- `bevy new`: create new projects from a template using `cargo-generate` ([#2](https://github.com/TheBevyFlock/bevy_cli/pull/2))
  - [`bevy_new_minimal`](https://github.com/TheBevyFlock/bevy_new_minimal) is the default template if none is specified ([#80](https://github.com/TheBevyFlock/bevy_cli/pull/80))
  - There are shortcuts for templates from [TheBevyFlock](https://github.com/TheBevyFlock). For example, `-t 2d` uses [`bevy_new_2d`](https://github.com/TheBevyFlock/bevy_new_2d) ([#82](https://github.com/TheBevyFlock/bevy_cli/pull/82))
- `bevy lint`: invoke the linter if `bevy_lint` is installed ([#4](https://github.com/TheBevyFlock/bevy_cli/pull/4))
- `bevy build` and `bevy run`: build and run your program with Bevy-specific configuration ([#76](https://github.com/TheBevyFlock/bevy_cli/pull/76), [#103](https://github.com/TheBevyFlock/bevy_cli/pull/103), [#102](https://github.com/TheBevyFlock/bevy_cli/pull/102), [#120](https://github.com/TheBevyFlock/bevy_cli/pull/120))
  - You can use `bevy build web` and `bevy run web` to build and run your program for the web using Wasm.
  - Web binaries can be optimized with `wasm-opt` ([#206](https://github.com/TheBevyFlock/bevy_cli/pull/206), [#430](https://github.com/TheBevyFlock/bevy_cli/pull/430))
  - You can pass `--bundle` to pack all files needed for the web into a single folder ([#195](https://github.com/TheBevyFlock/bevy_cli/pull/195))
- `bevy completions`: generate terminal auto-complete scripts for a variety of shells ([#265](https://github.com/TheBevyFlock/bevy_cli/pull/265))
- The CLI can be configured with `[package.metadata.bevy_cli]` ([#331](https://github.com/TheBevyFlock/bevy_cli/pull/331), [#355](https://github.com/TheBevyFlock/bevy_cli/pull/355), [#351](https://github.com/TheBevyFlock/bevy_cli/pull/351))
