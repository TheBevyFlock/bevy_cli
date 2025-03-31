# How to Parse Method Calls

A method call is a kind of expression, and can come in both receiver and qualified form:

```rust
// Both of these do the same thing!
receiver.method(args);
Struct::method(&receiver, args);
```

In order to parse a method call, you must first have an [`Expr`]. In this case we'll first implement `LateLintPass`'s `check_expr()` method, but you can get an [`Expr`] through several other means:

```rust
impl<'tcx> LateLintPass<'tcx> for MyLintPass {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        // ...
    }
}
```

Once you have an [`Expr`], you can parse method calls with the [`MethodCall`] utility:

```rust
use crate::utils::hir_parse::MethodCall;

fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
    // Extract a `MethodCall` from the `Expr` if it is a method call.
    if let Some(MethodCall { receiver, args, .. }) = MethodCall::try_from(cx, expr) {
        // ...
    }
}
```

`MethodCall::try_from()` returns an `Option`, and will only be `Some` if the expression was actually a method call. Take a look at `MethodCall`'s fields to see what properties are available.

> [!CAUTION]
>
> Although [`ExprKind::MethodCall`] does exist, it does not support qualified method syntax. You should avoid it if possible.
>
> [`ExprKind::MethodCall`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/enum.ExprKind.html#variant.MethodCall

[`Expr`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/struct.Expr.html
[`MethodCall`]: ../../src/utils/hir_parse.rs
