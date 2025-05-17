# Detecting `bevy_lint`

The linter passes `--cfg bevy_lint` when it checks your code, allowing you to detect it:

```rust,ignore
// Conditionally include this function only when `bevy_lint` is used.
#[cfg(bevy_lint)]
fn foo() {
    // ...
}

// Conditionally add an attribute only when `bevy_lint` is used.
#[cfg_attr(bevy_lint, ...)]
struct Foo;
```

If you use this, you may also need to register `bevy_lint` as a valid `cfg` flag in your `Cargo.toml`:

```toml
[lints.rust]
unexpected_cfg = { level = "warn", check-cfg = ["cfg(bevy_lint)"] }
```
