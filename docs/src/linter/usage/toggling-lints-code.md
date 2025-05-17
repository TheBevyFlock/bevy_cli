# Toggling Lints in Code

It is possible to set lint levels on a case-by-case basis inside your code, but it requires a few more steps than [setting the levels for the entire crate in `Cargo.toml`](toggling-lints-cargo-toml.md). First, you must [register `bevy` as a tool](register-bevy-tool.md). Not doing so will cause `#[allow(bevy::lint_name)]` and related attributes to fail to compile.

Once `bevy` is registered, you can toggle lints throughout your code, as long as they too are behind `#[cfg_attr(bevy_lint, ...)]`:

```rust,ignore
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]

// Enable the `pedantic` lint group, which is off by default.
#![cfg_attr(bevy_lint, warn(bevy::pedantic))]

// Deny panicking Bevy methods in this system when a non-panicking alternatives exist.
#[cfg_attr(bevy_lint, deny(bevy::panicking_methods))]
fn my_critical_system(world: &mut World) {
    // ...
}
```

There are several other ways to toggle lints, although some have varying levels of support:

|Method|Support|Additional Information|
|-|-|-|
|`[package.metadata.bevy_lint]` in `Cargo.toml`|✅|See [Toggling Lints in `Cargo.toml`](toggling-lints-cargo-toml.md).|
|`[workspace.metadata.bevy_lint]` in `Cargo.toml`|✅|See [Toggling Lints in `Cargo.toml`](toggling-lints-cargo-toml.md).|
|`#[allow(...)]` and related|✅|Must be behind `#[cfg_attr(bevy_lint, ...)]` on stable Rust.|
|`[lints.bevy]` in `Cargo.toml`|⚠️|Nightly only because `#[register_tool(bevy)]` must always be enabled. Prints a warning each time `cargo` is run.|
|`[workspace.lints.bevy]` in `Cargo.toml`|⚠️|Same as `[lints.bevy]`.|
|`RUSTFLAGS="-A bevy::lint"`|❌|`RUSTFLAGS` applies to dependencies, but they do not have `#[register_tool(bevy)]`.|
