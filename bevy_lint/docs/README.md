# Contributor's Guide

Thanks for your interest in contributing to `bevy_lint`! Please feel free to skim through the following table of contents:

- [Tutorials](tutorials/)
- [How-to](how-to/)
    - [Setting up your Editor](how-to/editor.md)
    - [How to Work with Types](how-to/types.md)
    - [How to Parse Method Calls](how-to/methods.md)
    - [How to Release `bevy_lint`](how-to/release.md)
    - [Bump to a Newer Version of Rust](how-to/bump-rust.md)
- [Reference](reference/)
    - [Lint Module Docs](reference/lint-module-docs.md)
    - [Macro-Generated Code](reference/macros.md)
    - [Coding Conventions and Best Practices](reference/conventions.md)
- [Explanations](explanations/)

These docs follow [Divio's Documentation System](https://docs.divio.com/documentation-system/).

> [!IMPORTANT]
>
> This is the documentation for _contributing_ to `bevy_lint`. If you want to learn how to _use_ `bevy_lint` instead, please view the live documentation [here](https://thebevyflock.github.io/bevy_cli/api/bevy_lint/) or see the main [`README.md`](../README.md).

## Additional Resources

⭐️ = Recommended Reading

- [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/)
    - [Debugging the compiler](https://rustc-dev-guide.rust-lang.org/compiler-debugging.html) (not all sections apply)
    - [Overview of the compiler](https://rustc-dev-guide.rust-lang.org/overview.html)
    - [Queries: demand-driven compilation](https://rustc-dev-guide.rust-lang.org/query.html)
    - [Memory Management in Rustc](https://rustc-dev-guide.rust-lang.org/memory.html)
    - ⭐️ [The HIR](https://rustc-dev-guide.rust-lang.org/hir.html)
    - [`rustc_driver` and `rustc_interface`](https://rustc-dev-guide.rust-lang.org/rustc-driver/intro.html)
    - ⭐️ [Errors and Lints](https://rustc-dev-guide.rust-lang.org/rustc-driver/intro.html)
    - ⭐️ [The `ty` module: representing types](https://rustc-dev-guide.rust-lang.org/ty.html)
    - [Glossary](https://rustc-dev-guide.rust-lang.org/appendix/glossary.html)
    - [Code Index](https://rustc-dev-guide.rust-lang.org/appendix/code-index.html)
    - [Humor in Rust](https://rustc-dev-guide.rust-lang.org/appendix/humorust.html) (not actually relevant)
- [`rustc` API Docs](https://doc.rust-lang.org/nightly/nightly-rustc/)
    - ⭐️ [`clippy_utils`](https://doc.rust-lang.org/nightly/nightly-rustc/clippy_utils/index.html)
        - ⭐️ [`match_type()`](https://doc.rust-lang.org/nightly/nightly-rustc/clippy_utils/ty/fn.match_type.html)
        <!-- As of 2025-02-21, Clippy hasn't synchronized with `rustc` yet, so `ty_from_hir_ty()` isn't in the docs. To work around this, we link to the docs.rs version. -->
        - ⭐️ [`ty_from_hir_ty()`](https://docs.rs/clippy_utils/latest/clippy_utils/ty/fn.ty_from_hir_ty.html)
        - ⭐️ [`span_lint()`](https://doc.rust-lang.org/nightly/nightly-rustc/clippy_utils/diagnostics/fn.span_lint.html)
        - ⭐️ [`snippet()`](https://doc.rust-lang.org/nightly/nightly-rustc/clippy_utils/source/fn.snippet.html)
    - [`rustc_driver`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_driver/index.html)
        - [`run_compiler()`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_driver/fn.run_compiler.html)
        - [`Callbacks`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_driver/trait.Callbacks.html)
    - [`rustc_hir`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/index.html)
        - ⭐️ [`DefId`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/def_id/struct.DefId.html)
        - [`LocalDefId`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/def_id/struct.LocalDefId.html)
        - [`HirId`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir_id/struct.HirId.html)
        - ⭐️ [`Expr`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/struct.Expr.html)
        - [`Item`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/struct.Item.html)
        - [`Path`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/struct.Path.html)
        - [`Ty`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/hir/struct.Ty.html) (Not to be confused with `rustc_middle::ty::Ty`)
    - [`rustc_span`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_span/index.html)
    - [`rustc_lint`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_lint/index.html)
        - ⭐️ [`LateLintPass`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_lint/trait.LateLintPass.html)
        - ⭐️ [`LateContext`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_lint/struct.LateContext.html)
    - [`rustc_lint_defs`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_lint_defs/index.html)
    - [`rustc_middle`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/index.html)
        - ⭐️ [`TyCtxt`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/context/struct.TyCtxt.html)
        - ⭐️ [`Ty`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.Ty.html) (Not to be confused with HIR `Ty`)
        - [`Map`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/hir/map/struct.Map.html) (HIR)
- [Clippy Development](https://doc.rust-lang.org/stable/clippy/development/index.html)
    - ⭐️ [Lint passes](https://doc.rust-lang.org/stable/clippy/development/lint_passes.html)
    - ⭐️ [Emitting a lint](https://doc.rust-lang.org/stable/clippy/development/emitting_lints.html)
    - ⭐️ [Type Checking](https://doc.rust-lang.org/stable/clippy/development/type_checking.html)
    - ⭐️ [Dealing with macros and expansions](https://doc.rust-lang.org/stable/clippy/development/macro_expansions.html)
    - [Common tools for writing lints](https://doc.rust-lang.org/stable/clippy/development/common_tools_writing_lints.html)
