# How to Parse Methods

There are two kinds of method call syntax, receiver and qualified form:

```rust
// Both of these do the same thing!
receiver.method(args);
Struct::method(&receiver, args);
```

Simply matching [`ExprKind::MethodCall`] does not account for the qualified form. In order to handle both types, the [`MethodCall`] utility should be used instead:

```rust
pub struct MethodCall<'tcx> {
    pub method_path: &'tcx PathSegment<'tcx>,
    pub receiver: &'tcx Expr<'tcx>,
    pub args: &'tcx [Expr<'tcx>],
    pub span: Span,
    pub is_fully_qulified: bool,
}
```

It has a very similar type definition to [`ExprKind::MethodCall`], and can be created from an [`Expr`] with `MethodCall::try_from()`:

```rust
fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
    if let Some(MethodCall { receiver, args, .. }) = MethodCall::try_from(cx, expr) {
        // ...
    }
}
```

[`ExprKind::MethodCall`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/enum.ExprKind.html#variant.MethodCall
[`MethodCall`]: ../../src/utils/hir_parse.rs
[`Expr`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/struct.Expr.html
