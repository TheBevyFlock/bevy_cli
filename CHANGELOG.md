# Changelog

All notable user-facing changes to the **Bevy CLI** will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## Unreleased

**All Changes**: [`cli-v0.1.0-alpha.1...main`](https://github.com/TheBevyFlock/bevy_cli/compare/cli-v0.1.0-alpha.1...main)

### Added

- Unstable support for building and running web apps using **Wasm multi-threading** features
  - Use `bevy run web --unstable multi-threading` to run an app using multi-threaded Wasm
  - Can be configured in `Cargo.toml` by setting `package.metadata.bevy_cli.unstable.web-multi-threading = true`
  - Only available when compiling the CLI with `unstable` feature enabled
  - Requires a nightly Rust toolchain
  - Bevy doesn't natively implement multi-threaded Wasm, so you have to implement it yourself or use a plugin that makes use of it

### Changed

- You can now customize the flags passed to `wasm-opt` in both CLI and `Cargo.toml`. Simply pass a list of flags you want to use, e.g. `--wasm-opt=-Oz --wasm-opt=--enable-bulk-memory` in the CLI or `wasm-opt = ["-Oz", "--enable-bulk-memory"]` in the config.

- `bevy run web` and `bevy build web -b` now support [JS snippets](https://rustwasm.github.io/wasm-bindgen/reference/js-snippets.html) ([#527](https://github.com/TheBevyFlock/bevy_cli/pull/527))

- `rustflags` specified in the [`cargo` configuration](https://doc.rust-lang.org/cargo/reference/config.html#configuration) are respected and added with those defined in `Cargo.toml` ([#540](https://github.com/TheBevyFlock/bevy_cli/pull/540))

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
