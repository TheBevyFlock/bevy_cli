# Toggling Lints in `Cargo.toml`

You can set the default level for lints in a `Cargo.toml` using the `[package.metadata.bevy_lint]` table:

```toml
[package.metadata.bevy_lint]
# Make the `missing_reflect` lint a warning.
missing_reflect = "warn"
# Make the `panicking_methods` lint an error that cannot be `#[allow(...)]`d.
panicking_methods = { level = "forbid" }
```

You can configure lints for an entire workspace by using `[workspace.metadata.bevy_lint]` in the root `Cargo.toml` instead:

```toml
[workspace.metadata.bevy_lint]
# Enable the entire `pedantic` lint group, and make them all warnings.
pedantic = "warn"
```

Crate lint configuration is merged with workspace lint configuration, with crate lint configuration taking priority.

Note that unlike with [Cargo's `[lints]` table](https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section), the `priority` field is not supported. Furthermore, if you wish to use `#[allow(...)]` and related attributes inside your code for Bevy-specific lints, please see [Toggling Lints in Code](toggling-lints-code.md).
