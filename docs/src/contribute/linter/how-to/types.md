# Work with Types

> **Note**
>
> This document assumes you are working with [`ty::Ty`], not [`rustc_hir::Ty`], unless explicitly stated otherwise.
>
> [`ty::Ty`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.Ty.html
> [`rustc_hir::Ty`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/struct.Ty.html

## Getting the Type of an Expression

It is possible to get the type of an expression ([`Expr`]) through [`TypeckResults`]:

```rust
fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &Expr<'tcx>) {
    let ty = cx.typeck_results().expr_ty(expr);
}
```

Usually you want to use [`expr_ty_adjusted()`](#getting-the-adjusted-type-of-an-expression) instead.

[`Expr`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/struct.Expr.html
[`TypeckResults`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/typeck_results/struct.TypeckResults.html

## Peeling References

Often you have the type that may be behind one or more references, such as `&&str` or `*mut [T]`, but you need to extract the underlying type (`str` and `[T]` in this case). You can do this by "peeling" references:

```rust
let peeled_ty = ty.peel_refs();
```

See [`Ty::peel_refs()`] for more information.

[`Ty::peel_refs()`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.Ty.html#method.peel_refs

## Getting the Adjusted Type of an Expression

The Rust compiler occasionally makes adjustments to types in order to support automatic dereferencing and type coercion. `TypeckResults::expr_ty()` ignores these adjustments, returning the type the user wrote. Usually this isn't desired, as you want to see the final coerced type, in which case you should use [`TypeckResults::expr_ty_adjusted()`] instead:

```rust
fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &Expr<'tcx>) {
    let ty = cx.typeck_results().expr_ty_adjusted(expr);
}
```

For example, you may be writing a lint to check when a user calls `str::contains()`. In order to catch the most cases, you want to also check for method calls on types that dereference into `str`, such as `String` and `Box<str>`. `expr_ty_adjusted()` lets you treat `String` and `Box<str>` as `str`:

```rust
// `expr_ty()` is `&str`, `expr_ty_adjusted()` is `&str`.
let a = "Hello, world!".contains("Hello");

// `expr_ty()` is `String`, `expr_ty_adjusted()` is `&str`.
let b = String::from("Hello, world!").contains("Hello");

// `expr_ty()` is `Box<&str>`, `expr_ty_adjusted()` is `&str`.
let c = Box::new("Hello, world!").contains("Hello");
```

For more information, see [`Adjustment`], [Type coercions], and [Method lookup].

[`TypeckResults::expr_ty_adjusted()`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TypeckResults.html#method.expr_ty_adjusted
[`Adjustment`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/adjustment/struct.Adjustment.html
[Type coercions]: https://doc.rust-lang.org/reference/type-coercions.html
[Method lookup]: https://rustc-dev-guide.rust-lang.org/method-lookup.html

## Checking for a Specific Type

Often you have a `Ty`, and want to check if it matches a specific hardcoded type, such as Bevy's [`App`]. You can check if a type matches a specific path using [`PathLookup::matches_ty()`](https://doc.rust-lang.org/nightly/nightly-rustc/clippy_utils/paths/struct.PathLookup.html#method.matches_ty). `PathLookup`s used by `bevy_lint` are placed in the `paths` module:

```rust
// Import the `PathLookup` for Bevy's `App` type.
use crate::paths::APP;

// Returns true if `ty` is an `App`.
if APP.matches_ty(cx, ty) {
    // ...
}
```

[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
[`match_type()`]: https://doc.rust-lang.org/nightly/nightly-rustc/clippy_utils/ty/fn.match_type.html

## Getting `ty::Ty` from `rustc_hir::Ty`

Often you'll have an [`rustc_hir::Ty`], but you need [`ty::Ty`]. This is a process known as _lowering_, and it is accomplished through the [`ty_from_hir_ty()`] function:

```rust
use clippy_utils::ty::ty_from_hir_ty;

fn check_ty(&mut self, cx: &LateContext<'tcx>, hir_ty: &rustc_hir::Ty<'tcx, AmbigArg>) {
    let ty = ty_from_hir_ty(cx, hir_ty.as_unambig_ty());
}
```

Also note that this conversion is one-directional and cannot be easily reversed. While [`rustc_hir::Ty`]s are associated with a specific span of code, [`ty::Ty`]s are not. For more information, please see [`rustc_hir::Ty` vs `ty::Ty`] from the `rustc` Dev Guide.

[`rustc_hir::Ty`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/struct.Ty.html
[`ty::Ty`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.Ty.html
[`ty_from_hir_ty()`]: https://doc.rust-lang.org/nightly/nightly-rustc/clippy_utils/ty/fn.ty_from_hir_ty.html
[`rustc_hir::Ty` vs `ty::Ty`]: https://rustc-dev-guide.rust-lang.org/ty.html#rustc_hirty-vs-tyty
