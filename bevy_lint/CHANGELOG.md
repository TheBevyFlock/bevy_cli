# Changelog

All notable user-facing changes to the **Bevy Linter** will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## Unreleased

**All Changes**: [`lint-v0.6.0...main`](https://github.com/TheBevyFlock/bevy_cli/compare/lint-v0.6.0...main)

## v0.6.0 - 2026-02-01

**All Changes**: [`lint-v0.5.0...lint-v0.6.0`](https://github.com/TheBevyFlock/bevy_cli/compare/lint-v0.5.0...lint-v0.6.0)

### Changed

- The linter now supports Bevy 0.18, but no longer supports Bevy 0.17 ([#703](https://github.com/TheBevyFlock/bevy_cli/pull/703))
- Bumped nightly toolchain to `nightly-2026-01-22` ([#729](https://github.com/TheBevyFlock/bevy_cli/pull/729))

## v0.5.0 - 2026-01-26

**All Changes**: [`lint-v0.4.0...lint-v0.5.0`](https://github.com/TheBevyFlock/bevy_cli/compare/lint-v0.4.0...lint-v0.5.0)

### Added

- Improve linter's diagnostics when it ICEs ([#517](https://github.com/TheBevyFlock/bevy_cli/pull/517))
- Added lint `missing_trait_for_unit_struct` to `restriction` ([#574](https://github.com/TheBevyFlock/bevy_cli/pull/574))
    - This checks for unit structs that do not implement `Copy`,`Clone` or `Default`.

### Changed

- The linter now supports Bevy 0.17, but no longer supports Bevy 0.16 ([#577](https://github.com/TheBevyFlock/bevy_cli/pull/577))
    - `insert_event_resource` lint got renamed to `insert_message_resource`.
    - `iter_current_update_events` lint got renamed to `iter_current_update_messages`.
- Bumped nightly toolchain to `nightly-2025-12-11` ([#697](https://github.com/TheBevyFlock/bevy_cli/pull/697))

### Fixed

- The `unit_in_bundle` lint no longer ICE's on projection types ([#659](https://github.com/TheBevyFlock/bevy_cli/pull/659))

### Removed

- Deprecated lint `insert_unit_bundle` was removed ([#724](https://github.com/TheBevyFlock/bevy_cli/pull/724))
    - Please use the `unit_in_bundle` lint instead.

## v0.4.0 - 2025-08-06

**All Changes**: [`lint-v0.3.0...lint-v0.4.0`](https://github.com/TheBevyFlock/bevy_cli/compare/lint-v0.3.0...lint-v0.4.0)

### Added

- You can now run `bevy_lint --fix` to auto-fix lints ([#505](https://github.com/TheBevyFlock/bevy_cli/pull/505))
- `bevy_lint` now has a custom `--help` screen to display options specific to the linter ([#505](https://github.com/TheBevyFlock/bevy_cli/pull/505))
- Added lints `update_schedule` and `fixed_update_schedule` to `restriction` ([#463](https://github.com/TheBevyFlock/bevy_cli/pull/463))
    - These can help restrict modules to only use the `Update` or `FixedUpdate` schedules. Useful for separating game and rendering logic!
- Added lint `camera_modification_in_fixed_update` to `nursery` ([#417](https://github.com/TheBevyFlock/bevy_cli/pull/417))
    - This catches cases where a camera is modified during `FixedUpdate`, which can cause laggy visuals.
- The Bevy CLI (Alpha) can now automatically install the linter for you ([#406](https://github.com/TheBevyFlock/bevy_cli/pull/406))
- It is now possible to use the linter without Rustup by specifying the `BEVY_LINT_SYSROOT` environmental variable ([#478](https://github.com/TheBevyFlock/bevy_cli/pull/478))
    - This should make it easier to use the linter with NixOS.
- Added opt-in support for caching `bevy_lint` in Github Actions ([#530](https://github.com/TheBevyFlock/bevy_cli/pull/530))
    - This can double the speed at which `bevy_lint` is installed in CI, so it is highly recommended to enable it [by following these instructions](https://thebevyflock.github.io/bevy_cli/linter/github-actions.html#caching)!
- Added docs on how to use `bevy_lint` with Rust-Analyzer ([#503](https://github.com/TheBevyFlock/bevy_cli/pull/503))
- Added docs for troubleshooting issues with `cranelift` and `sccache` ([#453](https://github.com/TheBevyFlock/bevy_cli/pull/453), [#522](https://github.com/TheBevyFlock/bevy_cli/pull/522))

### Changed

- The lint `insert_unit_bundle` has been renamed to `unit_in_bundle` because it now supports many more cases, not just `Commands::spawn()` ([#502](https://github.com/TheBevyFlock/bevy_cli/pull/502))
- Improved `unconventional_naming`'s diagnostics when encountering a `Plugin` ([#495](https://github.com/TheBevyFlock/bevy_cli/pull/495))
- The linter documentation has been moved to use `mdbook` instead of `rustdoc` ([#420](https://github.com/TheBevyFlock/bevy_cli/pull/420), [#436](https://github.com/TheBevyFlock/bevy_cli/pull/436))
    - You can find the [new docs here](https://thebevyflock.github.io/bevy_cli/linter).
    - The list of all lints is still generated using `rustdoc`, which you can find [here](https://thebevyflock.github.io/bevy_cli/api/bevy_lint/lints/).
- Bumped nightly toolchain to `nightly-2025-06-26` ([#507](https://github.com/TheBevyFlock/bevy_cli/pull/507))
    - This adds support for the latest Rust features, such as let-chains!
- You can now copy-and-paste most commands in the docs, without having to lookup the compatibility table ([#475](https://github.com/TheBevyFlock/bevy_cli/pull/475))
- You can now install specific commits of the linter with the Github Action ([#501](https://github.com/TheBevyFlock/bevy_cli/pull/501))
    - This will only work for commits newer than [`f38247d`](https://github.com/TheBevyFlock/bevy_cli/commit/f38247daea376c64919e1d09527acbbadb6df14b).

### Fixed

- The linter will no longer emit the `Plugin` / `SystemSet` span twice in `unconventional_naming` ([#495](https://github.com/TheBevyFlock/bevy_cli/pull/495))
- Some lints now support auto-dereference receiver methods ([#504](https://github.com/TheBevyFlock/bevy_cli/pull/504))
    - For example, `panicking_methods` now catches `Box<World>::resource()` where before it would silently pass.

## v0.3.0 - 2025-04-30

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
    - This makes it easier to see what lint group a lint is under in [the documentation](https://thebevyflock.github.io/bevy_cli/api/bevy_lint/). For example, in v0.2.0 if you wanted to view the `insert_unit_bundle` lint you would go to `bevy_lint::lints::insert_unit_bundle`, but in v0.3.0 you would go to `bevy_lint::lints::suspicious::insert_unit_bundle`. This signals that `insert_unit_bundle` is a `suspicious` lint.
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

## v0.2.0 - 2025-03-19

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

## v0.1.0 - 2024-11-17

**All Changes**: [`lint-v0.1.0`](https://github.com/TheBevyFlock/bevy_cli/commits/lint-v0.1.0)

### Added

- Lint `main_return_without_appexit` to `pedantic` ([#84](https://github.com/TheBevyFlock/bevy_cli/pull/84))
- Lint `insert_event_resource` to `suspicious` ([#86](https://github.com/TheBevyFlock/bevy_cli/pull/86))
- Lint groups `correctness`, `suspicious`, `complexity`, `performance`, `style`, `pedantic`, `restriction`, and `nursery` ([#98](https://github.com/TheBevyFlock/bevy_cli/pull/98))
    - These are based directly on [Clippy's Lint Groups](https://doc.rust-lang.org/stable/clippy/lints.html).
- Lints `panicking_query_methods` and `panicking_world_methods` to `restriction` ([#95](https://github.com/TheBevyFlock/bevy_cli/pull/95))
- Lint `plugin_not_ending_in_plugin` to `style` ([#111](https://github.com/TheBevyFlock/bevy_cli/pull/111))
- Lint `missing_reflect` to `restriction` ([#139](https://github.com/TheBevyFlock/bevy_cli/pull/139))
- Lint `zst_query` to `restriction` ([#168](https://github.com/TheBevyFlock/bevy_cli/pull/168))
