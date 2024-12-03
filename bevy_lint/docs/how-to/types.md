# How to Work with Types

> [!NOTE]
>
> This document assumes you are working with [`rustc_middle::ty::Ty`], not [`rustc_hir::Ty`], unless explicitly stated otherwise.
>
> [`rustc_middle::ty::Ty`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.Ty.html

## Getting the Type of an Expression

It is possible to get the type of an expression ([`Expr`]) through [`TypeckResults`]:

```rust
fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &Expr<'tcx>) {
    let ty = cx.typeck_results().expr_ty(expr);
}
```

[`Expr`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/struct.Expr.html
[`TypeckResults`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/typeck_results/struct.TypeckResults.html

## Peeling References

Often you have the type that may be behind one or more references, such as `&&str` or `*mut [T]`, but you need to extract the underlying type (`str` and `[T]` in this case). You can do this by "peeling" references:

```rust
let peeled_ty = ty.peel_refs();
```

See [`Ty::peel_refs()`] for more information.

[`Ty::peel_refs()`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.Ty.html#method.peel_refs
