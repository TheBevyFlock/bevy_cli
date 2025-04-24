# Migration Guide

Occasionally, changes are made to `bevy_lint` that may break existing projects, or majorly changes how it is intended to be used. This document provides a guide recommending how to upgrade your existing project to a newer version of the linter.

To actually install the new version of the linter, please see [the docs] and [the releases page]. Note that some changes listed here are optional (and will be explicitly marked as such). If you ever run into issues while upgrading, please feel free to [submit an issue].

[the docs]: https://thebevyflock.github.io/bevy_cli/bevy_lint/index.html
[the releases page]: https://github.com/TheBevyFlock/bevy_cli/releases
[submit an issue]: https://github.com/TheBevyFlock/bevy_cli/issues

## v0.2.0 to v0.3.0

### [Bevy 0.16 Support](https://github.com/TheBevyFlock/bevy_cli/pull/323)

The linter now supports Bevy 0.16, but no longer supports Bevy 0.15. You may still be able to run the linter successfully on Bevy 0.15 projects, but no guarantee is made on stability or correctness.

To migrate your code base to Bevy 0.16, please see the [release post][bevy 0.16 release post] and [migration guide][bevy 0.16 migration guide].

[bevy 0.16 release post]: TODO
[bevy 0.16 migration guide]: https://bevyengine.org/learn/migration-guides/0-15-to-0-16/

### [Bumped Nightly Toolchain to `nightly-2025-04-03`](https://github.com/TheBevyFlock/bevy_cli/pull/278)

The linter now requires the `nightly-2025-04-03` Rustup toolchain to be installed, instead of `nightly-2025-02-20`. The supported Rust language version is now 1.88.0 instead of the previous 1.87.0.

For more information on how to install this toolchain and its required components, please see the [linter docs].

### [Removed `plugin_not_ending_in_plugin`](https://github.com/TheBevyFlock/bevy_cli/pull/345)

The `plugin_not_ending_in_plugin` lint has been removed in favor of the new `unconventional_naming` lint. `unconventional_naming` offers the same checks as `plugin_not_ending_in_plugin`, but now supports checking `SystemSet`s as well.

If you reference `plugin_not_ending_in_plugin` in your code, a new warning will be emitted suggesting you rename it to `unconventional_naming`.

## v0.1.0 to v0.2.0

### [Bevy 0.15 Support](https://github.com/TheBevyFlock/bevy_cli/pull/191)

The linter now supports Bevy 0.15, but no longer supports Bevy 0.14. You may still be able to run the linter successfully on Bevy 0.14 projects, but no guarantee is made on stability or correctness.

To migrate your code base to Bevy 0.15, please see the [release post][bevy 0.15 release post] and [migration guide][bevy 0.15 migration guide].

[bevy 0.15 release post]: https://bevyengine.org/news/bevy-0-15/
[bevy 0.15 migration guide]: https://bevyengine.org/learn/migration-guides/0-14-to-0-15/

### [Bumped Nightly Toolchain to `nightly-2025-02-20`](https://github.com/TheBevyFlock/bevy_cli/pull/278)

The linter now requires the `nightly-2025-02-20` Rustup toolchain to be installed, instead of `nightly-2024-11-14`. The supported Rust language version is now 1.87.0[^rust-2024] instead of the previous 1.84.0.

For more information on how to install this toolchain and its required components, please see the [linter docs].

[linter docs]: https://thebevyflock.github.io/bevy_cli/bevy_lint/index.html

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
[`nursery`]: https://thebevyflock.github.io/bevy_cli/bevy_lint/groups/static.NURSERY.html
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
