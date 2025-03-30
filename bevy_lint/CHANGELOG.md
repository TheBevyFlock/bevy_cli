# Changelog

All notable user-facing changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## [Unreleased]

**All Changes**: [`lint-v0.2.0...main`](https://github.com/TheBevyFlock/bevy_cli/compare/lint-v0.2.0...main)

### Added

- Lint `iter_current_update_events` to `suspicious` ([#314](https://github.com/TheBevyFlock/bevy_cli/pull/314))

## [v0.2.0] - 2025-03-19

**All Changes**: [`lint-v0.1.0...lint-v0.2.0`](https://github.com/TheBevyFlock/bevy_cli/compare/lint-v0.1.0...lint-v0.2.0)

### Added

- Lint `borrowed_reborrowable` to `pedantic` ([#164](https://github.com/TheBevyFlock/bevy_cli/pull/164))
- Lint `insert_unit_bundle` to `suspicious` ([#210](https://github.com/TheBevyFlock/bevy_cli/pull/210))
- Lint configuration in `Cargo.toml` ([#251](https://github.com/TheBevyFlock/bevy_cli/pull/251))
- Support for `bevy_lint --version` ([#257](https://github.com/TheBevyFlock/bevy_cli/pull/257))
- Support for qualified method syntax in several lints ([#253](https://github.com/TheBevyFlock/bevy_cli/pull/253))
- Lint `duplicate_bevy_dependencies` ([#280](https://github.com/TheBevyFlock/bevy_cli/pull/280))

### Changed

- The linter now supports Bevy 0.15, but no longer supports Bevy 0.14 ([#191](https://github.com/TheBevyFlock/bevy_cli/pull/191))
    - Eventually the linter will support multiple versions of Bevy at the same time. Please see [#138](https://github.com/TheBevyFlock/bevy_cli/issues/138) for more information.
- Bumped nightly toolchain to `nightly-2025-02-20` ([#278](https://github.com/TheBevyFlock/bevy_cli/pull/278))
- Lowered `zst_query` lint from `restriction` to `nursery` ([#261](https://github.com/TheBevyFlock/bevy_cli/pull/261))
    - `zst_query` does not respect `QueryData::Item`, meaning it is broken for queries like `Has<T>` and `AnyOf<T>`. Please see [#279](https://github.com/TheBevyFlock/bevy_cli/issues/279) for more information.
- Merged `panicking_query_methods` and `panicking_world_methods` into a single lint: `panicking_methods` ([#271](https://github.com/TheBevyFlock/bevy_cli/pull/271))

### Fixed

- `rustc_driver.dll` not found on Windows ([#281](https://github.com/TheBevyFlock/bevy_cli/pull/281))
    - `bevy_lint` should now work on Windows, as it was previously broken by this bug.

## [v0.1.0] - 2024-11-17

**All Changes**: [`17834eb...lint-v0.1.0`](https://github.com/TheBevyFlock/bevy_cli/compare/17834eb...lint-v0.1.0)

### Added

- Lint `main_return_without_appexit` to `pedantic` ([#84](https://github.com/TheBevyFlock/bevy_cli/pull/84))
- Lint `insert_event_resource` to `suspicious` ([#86](https://github.com/TheBevyFlock/bevy_cli/pull/86))
- Lint groups `correctness`, `suspicious`, `complexity`, `performance`, `style`, `pedantic`, `restriction`, and `nursery` ([#98](https://github.com/TheBevyFlock/bevy_cli/pull/98))
    - These are based directly on [Clippy's Lint Groups](https://doc.rust-lang.org/stable/clippy/lints.html).
- Lints `panicking_query_methods` and `panicking_world_methods` to `restriction` ([#95](https://github.com/TheBevyFlock/bevy_cli/pull/95))
- Lint `plugin_not_ending_in_plugin` to `style` ([#111](https://github.com/TheBevyFlock/bevy_cli/pull/111))
- Lint `missing_reflect` to `restriction` ([#139](https://github.com/TheBevyFlock/bevy_cli/pull/139))
- Lint `zst_query` to `restriction` ([#168](https://github.com/TheBevyFlock/bevy_cli/pull/168))
