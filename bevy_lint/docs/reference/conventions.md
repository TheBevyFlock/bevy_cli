# Coding Conventions and Best Practices

This document contains a list of conventions that `bevy_lint`'s code follows to ensure consistency across the project. Beyond this document, the project uses [Clippy], [`rustfmt`], and [`typos`] in CI, and it follows the majority of the [Rust API Guidelines].

[Clippy]: https://doc.rust-lang.org/clippy/
[`rustfmt`]: https://github.com/rust-lang/rustfmt
[`typos`]: https://github.com/crate-ci/typos
[Rust API Guidelines]: https://rust-lang.github.io/api-guidelines/

## Avoid `HasSession`

[`HasSession`] is a trait in `clippy_utils` intended to make their API more friendly. `HasSession` provides the `sess()` method on `TyCtxt`, `LateContext`, and a few other types. `HasSession::sess()` should not be used within `bevy_lint` because [`Session`] is trivial to get without it, and `HasSession` requires an extra import.

```rust
use clippy_utils::source::HasSession;

fn late_context<'tcx>(cx: LateContext<'tcx>) {
    let sess = cx.sess();
}

fn ty_ctxt<'tcx>(tcx: TyCtxt<'tcx>) {
    let sess = tcx.sess();
}
```

Use instead:

```rust
fn late_context<'tcx>(cx: LateContext<'tcx>) {
    let sess = cx.tcx.sess;
}

fn ty_ctxt<'tcx>(tcx: TyCtxt<'tcx>) {
    let sess = tcx.sess;
}
```

[`HasSession`]: https://doc.rust-lang.org/nightly/nightly-rustc/clippy_utils/source/trait.HasSession.html
[`Session`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_session/session/struct.Session.html
