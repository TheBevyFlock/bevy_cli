# Registering `bevy` as a Tool

When you run `bevy_lint` on a project, `rustc` knows an exact list of all `bevy::` lints registered. With this it can detect that `bevy::missing_reflect` is valid and `bevy::uh_oh` isn't, and emit a corresponding warning.

When you run normal `cargo check`, however, it does not know about _any_ `bevy::` lints. In order to avoid erroring on _all_ usages of `bevy::`, but to still provide good diagnostics on typos, the `#![register_tool(...)]` attribute was introduced.

```rust,ignore
// Note that this is nightly-only. We'll get to that in a second!
#![register_tool(bevy)]
```

Using `#![register_tool(bevy)]` tells the compiler that `bevy` is a valid name in attributes, even if it does not know what `bevy` is.[^rustfmt-skip] When `cargo check` now runs over a project with `#[warn(bevy::lint_name)]`, it will simply skip it instead of emitting an error. (But running `bevy_lint` will still detect and check this attribute as normal.)

[^rustfmt-skip]: If you've ever used `#[rustfmt::skip]` in your code, this is how `rustc` avoids erroring on it. However unlike the `bevy` namespace, `rustfmt` is registered automatically without a need for `#![register_tool(rustfmt)]` due to it being an official tool.

If you wish to refer to a `bevy` lint at all in your code (usually to [toggle it](toggling-lints-code.md)), you must add `#![register_tool(bevy)]` to each crate root. Unfortunately, `#![register_tool(...)]` is [currently unstable](https://doc.rust-lang.org/nightly/unstable-book/language-features/register-tool.html), meaning you need to add `#![feature(register_tool)]` to your code as well. This isn't an issue if you [detect when `bevy_lint` is enabled](detecting-bevy-lint.md), since it is guaranteed to check your code using nightly Rust.

```rust,ignore
// When `bevy_lint` is used, enable the `register_tool` feature and register `bevy` as a tool.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
```

> **Tip**
>
> If your project already uses nightly Rust, you can forego the `#[cfg_attr(bevy_lint, ...)]` attributes and write `#![feature(register_tool)]` and `#![register_tool(bevy)]` directly! Cool!
