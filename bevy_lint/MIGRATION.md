# Migration Guide

Occasionally changes are made to the **Bevy Linter** that may break existing projects, or majorly change how it is intended to be used. This document provides a guide recommending how to upgrade your existing project to a newer version of the linter.

To actually install the new version of the linter, please see [the docs] and [the releases page]. Note that some changes listed here are optional (and will be explicitly marked as such). If you ever run into issues while upgrading, please feel free to [submit an issue].

[the docs]: https://thebevyflock.github.io/bevy_cli/linter/index.html
[the releases page]: https://github.com/TheBevyFlock/bevy_cli/releases
[submit an issue]: https://github.com/TheBevyFlock/bevy_cli/issues

## v0.5.0 to v0.6.0

### [Bevy 0.18 Support](https://github.com/TheBevyFlock/bevy_cli/pull/703)

The linter now supports Bevy 0.18, but no longer supports Bevy 0.17.
To migrate your code base to Bevy 0.18, please see the [release post][bevy 0.18 release post] and [migration guide][bevy 0.18 migration guide].

[bevy 0.18 release post]: https://bevy.org/news/bevy-0-18/
[bevy 0.18 migration guide]: https://bevy.org/learn/migration-guides/0-17-to-0-18/

### [Bumped Nightly Toolchain to `nightly-2026-01-22`](https://github.com/TheBevyFlock/bevy_cli/pull/729)

`bevy_lint` now requires the `nightly-2026-01-22` toolchain, which supports Rust 1.95.0. You may uninstall the old `nightly-2025-12-11` toolchain and install the new toolchain using Rustup:

```sh
rustup toolchain uninstall nightly-2025-12-11

rustup toolchain install nightly-2026-01-22 \
    --component rustc-dev \
    --component llvm-tools
```

## v0.4.0 to v0.5.0

### [Bevy 0.17 Support](https://github.com/TheBevyFlock/bevy_cli/pull/577)

The linter now supports Bevy 0.17, but no longer supports Bevy 0.16.
To migrate your code base to Bevy 0.17, please see the [release post][bevy 0.17 release post] and [migration guide][bevy 0.17 migration guide].

[bevy 0.17 release post]: https://bevy.org/news/bevy-0-17/
[bevy 0.17 migration guide]: https://bevy.org/learn/migration-guides/0-16-to-0-17/

### [Renamed Lints that Target Buffered Events](https://github.com/TheBevyFlock/bevy_cli/pull/577)

- `insert_event_resource` lint was renamed to `insert_message_resource`
- `iter_current_update_events` lint was renamed to `iter_current_update_messages`

In Bevy 0.17, `Event` now exclusively refers to observers. Buffered events, using `EventReader` and `EventWriter`, are now referred to as [`Message`]s, with [`MessageReader`] and [`MessageWriter`]. The lints related to buffered events have been renamed to reflect this change.

[`Message`]: https://docs.rs/bevy/0.17.3/bevy/ecs/message/trait.Message.html
[`MessageWriter`]: https://docs.rs/bevy/0.17.3/bevy/ecs/message/struct.MessageWriter.html
[`MessageReader`]: https://docs.rs/bevy/0.17.3/bevy/ecs/message/struct.MessageReader.html

### [Bumped Nightly Toolchain to `nightly-2025-12-11`](https://github.com/TheBevyFlock/bevy_cli/pull/697)

`bevy_lint` now requires the `nightly-2025-12-11` toolchain, which supports Rust 1.94.0. You may uninstall the old `nightly-2025-06-26` toolchain and install the new toolchain using Rustup:

```sh
rustup toolchain uninstall nightly-2025-06-26

rustup toolchain install nightly-2025-12-11 \
    --component rustc-dev \
    --component llvm-tools
```

## v0.3.0 to v0.4.0

### [Bumped Nightly Toolchain to `nightly-2025-06-26`](https://github.com/TheBevyFlock/bevy_cli/pull/507)

`bevy_lint` now requires the `nightly-2025-06-26` toolchain, which supports Rust 1.90.0. You may uninstall the old `nightly-2025-04-03` toolchain and install the new toolchain using Rustup:

```sh
rustup toolchain uninstall nightly-2025-04-03

rustup toolchain install nightly-2025-06-26 \
    --component rustc-dev \
    --component llvm-tools
```

### [`insert_unit_bundle` Has Been Renamed to `unit_in_bundle`](https://github.com/TheBevyFlock/bevy_cli/pull/502)

The `unit_in_bundle` lint is a much more powerful version of the older `insert_unit_bundle` lint, as it now works for many more functions instead of just `Commands::spawn()`. If you reference `insert_unit_bundle` in your project, you will need to rename it to `unit_in_bundle`.

## v0.2.0 to v0.3.0

### [Bevy 0.16 Support](https://github.com/TheBevyFlock/bevy_cli/pull/323)

The linter now supports Bevy 0.16, but no longer supports Bevy 0.15. You may still be able to run the linter successfully on Bevy 0.15 projects, but no guarantee is made on stability or correctness.

To migrate your code base to Bevy 0.16, please see the [release post][bevy 0.16 release post] and [migration guide][bevy 0.16 migration guide].

[bevy 0.16 release post]: https://bevy.org/news/bevy-0-16/
[bevy 0.16 migration guide]: https://bevy.org/learn/migration-guides/0-15-to-0-16/

### [Bumped Nightly Toolchain to `nightly-2025-04-03`](https://github.com/TheBevyFlock/bevy_cli/pull/278)

The linter now requires the `nightly-2025-04-03` Rustup toolchain to be installed, instead of `nightly-2025-02-20`. The supported Rust language version is now 1.88.0 instead of the previous 1.87.0.

For more information on how to install this toolchain and its required components, please see the [linter docs].

### [Removed `plugin_not_ending_in_plugin`](https://github.com/TheBevyFlock/bevy_cli/pull/345)

The `plugin_not_ending_in_plugin` lint has been removed in favor of the new `unconventional_naming` lint. `unconventional_naming` offers the same checks as `plugin_not_ending_in_plugin`, but now supports checking `SystemSet`s as well.

If you reference `plugin_not_ending_in_plugin` in your code, a new warning will be emitted suggesting you rename it to `unconventional_naming`.

### [Created Github Action to install `bevy_lint`](https://github.com/TheBevyFlock/bevy_cli/pull/380) (Optional)

If you were manually installing `bevy_lint` in CI by following the [installation instructions](https://thebevyflock.github.io/bevy_cli/linter/install.html), you can now replace it with the new action:

```yml
- name: Install `bevy_lint`
  uses: TheBevyFlock/bevy_cli/bevy_lint@lint-v0.3.0

- name: Run `bevy_lint`
  run: bevy_lint --workspace
```

## v0.1.0 to v0.2.0

### [Bevy 0.15 Support](https://github.com/TheBevyFlock/bevy_cli/pull/191)

The linter now supports Bevy 0.15, but no longer supports Bevy 0.14. You may still be able to run the linter successfully on Bevy 0.14 projects, but no guarantee is made on stability or correctness.

To migrate your code base to Bevy 0.15, please see the [release post][bevy 0.15 release post] and [migration guide][bevy 0.15 migration guide].

[bevy 0.15 release post]: https://bevy.org/news/bevy-0-15/
[bevy 0.15 migration guide]: https://bevy.org/learn/migration-guides/0-14-to-0-15/

### [Bumped Nightly Toolchain to `nightly-2025-02-20`](https://github.com/TheBevyFlock/bevy_cli/pull/278)

The linter now requires the `nightly-2025-02-20` Rustup toolchain to be installed, instead of `nightly-2024-11-14`. The supported Rust language version is now 1.87.0[^rust-2024] instead of the previous 1.84.0.

For more information on how to install this toolchain and its required components, please see the [linter docs].

[linter docs]: https://thebevyflock.github.io/bevy_cli/linter/index.html

[^rust-2024]: As a nice side-effect, `bevy_lint` now officially supports [Rust 2024](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html).

### [Merged `panicking_query_methods` and `panicking_world_methods`](https://github.com/TheBevyFlock/bevy_cli/pull/271)

The `panicking_query_methods` and `panicking_world_methods` lints have been merged into a single lint: `panicking_methods`. This new lint has the same functionality as the two previous lints combined.

```rust
// v0.1.0
#[cfg_attr(bevy_lint, deny(bevy::panicking_query_methods))]
fn critical_system(query: Query<&MyComponent>) {
    // ...
}

// v0.2.0
#[cfg_attr(bevy_lint, deny(bevy::panicking_methods))]
fn critical_system(query: Query<&MyComponent>) {
    // ...
}
```

### [Lowered `zst_query` from `restriction` to `nursery`](https://github.com/TheBevyFlock/bevy_cli/pull/261)

A critical bug was found in `zst_query` where it would incorrectly warn on certain queries that _do_ actually query data, such as [`Has<T>`] and [`AnyOf<T>`]. As such, it has been temporarily moved to the [`nursery`] lint group, meaning that it is marked as unstable and may be removed.

Until [#279] is fixed, it is recommended to remove references of this lint from your project.

[`Has<T>`]: https://docs.rs/bevy/0.15.3/bevy/ecs/prelude/struct.Has.html
[`AnyOf<T>`]: https://docs.rs/bevy/0.15.3/bevy/ecs/prelude/struct.AnyOf.html
[`nursery`]: https://thebevyflock.github.io/bevy_cli/api/bevy_lint/lints/nursery/index.html
[#279]: https://github.com/TheBevyFlock/bevy_cli/issues/279

### [Lint Configuration in `Cargo.toml`](https://github.com/TheBevyFlock/bevy_cli/pull/251) (Optional)

It is now possible to configure lints in a crate's `Cargo.toml` instead of directly in its code.

```rust
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]

// You can delete these attributes from your crate root.
#![cfg_attr(bevy_lint, warn(bevy::pedantic))]
#![cfg_attr(bevy_lint, deny(bevy::panicking_methods))]
```

```toml
# And add them to `Cargo.toml` instead.
[package.metadata.bevy_lint]
pedantic = "warn"
panicking_methods = "deny"
```

Lint levels specified in `Cargo.toml` will be applied for the entire crate, but can still be overridden in your code.
