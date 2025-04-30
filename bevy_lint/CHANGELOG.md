# Changelog

All notable user-facing changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## [Unreleased]

**All Changes**: [`lint-v0.3.0...main`](https://github.com/TheBevyFlock/bevy_cli/compare/lint-v0.3.0...main)

## [v0.3.0] - 2025-04-30

**All Changes**: [`lint-v0.2.0...lint-v0.3.0`](https://github.com/TheBevyFlock/bevy_cli/compare/lint-v0.2.0...lint-v0.3.0)

### Added

- Lint `iter_current_update_events` to `suspicious` ([#314](https://github.com/TheBevyFlock/bevy_cli/pull/314))
- Lint `unconventional_naming` to `style` ([#345](https://github.com/TheBevyFlock/bevy_cli/pull/345))
    - `plugin_not_ending_in_plugin` has been merged into this new lint.
- A Github Action to automatically install the linter ([#380](https://github.com/TheBevyFlock/bevy_cli/pull/380))

### Changed

- The linter now supports Bevy 0.16, but no longer supports Bevy 0.15 ([#323](https://github.com/TheBevyFlock/bevy_cli/pull/323))
- Bumped nightly toolchain to `nightly-2025-04-03` ([#373](https://github.com/TheBevyFlock/bevy_cli/pull/373))
    - The linter now supports Rust 1.88.0.
- Moved lints into submodules for their corresponding lint groups ([#321](https://github.com/TheBevyFlock/bevy_cli/pull/321))
    - This makes it easier to see what lint group a lint is under in [the documentation](https://thebevyflock.github.io/bevy_cli/bevy_lint/). For example, in v0.2.0 if you wanted to view the `insert_unit_bundle` lint you would go to `bevy_lint::lints::insert_unit_bundle`, but in v0.3.0 you would go to `bevy_lint::lints::suspicious::insert_unit_bundle`. This signals that `insert_unit_bundle` is a `suspicious` lint.
- Moved lint group docs from `bevy_lint::groups` to their associated `bevy_lint::lints` submodules ([#328](https://github.com/TheBevyFlock/bevy_cli/pull/328))
- Code generated from external macros are no longer linted ([#263](https://github.com/TheBevyFlock/bevy_cli/pull/263))
    - External macros are macros that are defined in a separate crate from the one being linted. The output of these macros is skipped for all lints, as it was previously impossible to fix the warnings without an `#[allow(...)]` attribute.
- `missing_reflect` now emits machine-applicable suggestions if all fields in a type implement `PartialReflect` ([#389](https://github.com/TheBevyFlock/bevy_cli/pull/389))

### Removed

- Lint `plugin_not_ending_in_plugin` ([#345](https://github.com/TheBevyFlock/bevy_cli/pull/345))
    - This lint has been merged into the new `unconventional_naming` lint.

### Fixed

- `main_return_without_appexit` no longer fires if the `AppExit` is used ([#346](https://github.com/TheBevyFlock/bevy_cli/pull/346))
    - The goal of the lint is to encourage the `AppExit` to be handled, although returning it from `main()` is just one solution. This fix prevents the lint from yelling at you if you choose to handle it a different way, or simply choose to discard it with `let _ = app.run();`.
- Fixed the Rust version in the compatibility table for v0.2.0 ([#363](https://github.com/TheBevyFlock/bevy_cli/pull/363))

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
