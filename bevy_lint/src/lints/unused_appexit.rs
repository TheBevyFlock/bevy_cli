//! Checks for instances where `App::run()` is called but does not handle the returned `AppExit`.
//!
//! # Motivation
//!
//! `AppExit` is used to determine whether the `App` exited successful or due to an error (such as
//! when the render thread panics). Handling `AppExit` is useful for warning about errors that may
//! otherwise be silent.
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! fn main() {
//!     // `AppExit` is discarded, oh no!
//!     App::new().run();
//! }
//! ```
//!
//! The easiest method to fix this lint is to return `AppExit` from the `main()` function. This
//! sets the exit code of the process to 1 on an error:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! fn main() -> AppExit {
//!     // Note the removed semicolon.
//!     App::new().run()
//! }
//! ```
//!
//! You may also choose to emit the error directly, such as when you're compiling for WASM where
//! the exit code is not visible nor meaningful:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! fn main() -> AppExit {
//!     let app_exit = App::new().run();
//!
//!     if let AppExit::Error(code) = app_exit {
//!         error!("App exited with an error, exit code {code}.");
//!     }
//!
//!     app_exit
//! }
//! ```
//!
//! If you truly wish to silence the lint, you can store `AppExit` in `_`:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! fn main() {
//!     let _ = App::new().run();
//! }
//! ```

use clippy_utils::{
    diagnostics::span_lint_and_then, is_expr_used_or_unified, source::snippet_opt, sym,
    ty::match_type,
};
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind, HirId, Item, ItemKind, Node};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::impl_lint_pass;
use rustc_span::Symbol;

use crate::declare_bevy_lint;

declare_bevy_lint! {
    pub UNUSED_APPEXIT,
    PEDANTIC,
    "called `App::run()` without handling the returned `AppExit`",
}

#[derive(Clone, Copy)]
pub struct UnusedAppExit {
    /// A cached [`Symbol`] representing the interned string `"run"`.
    run_symbol: Symbol,
}

impl Default for UnusedAppExit {
    fn default() -> Self {
        Self {
            run_symbol: sym!(run),
        }
    }
}

impl_lint_pass! {
    UnusedAppExit => [UNUSED_APPEXIT.lint]
}

impl<'tcx> LateLintPass<'tcx> for UnusedAppExit {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &Expr<'tcx>) {
        // Find a method call that matches `.run()`.
        if let ExprKind::MethodCall(path, src, _, method_span) = expr.kind
            && path.ident.name == self.run_symbol
        {
            // Get the type of `src` for `src.run()`. We peel away all references because both
            // `App` and `&mut App` are allowed.
            let ty = cx.typeck_results().expr_ty(src).peel_refs();

            // If `src` is a Bevy `App` and the returned `AppExit` is not used, emit the lint. The
            // biggest player in this check is the `is_expr_used_or_unified()` function, which
            // checks if `AppExit` is handled.
            if match_type(cx, ty, &crate::paths::APP) && !is_expr_used_or_unified(cx.tcx, expr) {
                span_lint_and_then(
                    cx,
                    UNUSED_APPEXIT.lint,
                    method_span,
                    UNUSED_APPEXIT.lint.desc,
                    |diag| {
                        let snippet = snippet_opt(cx, expr.span);

                        diag.note("`App::run()` returns `AppExit`, which is used to determine whether the app exited successfully or not");

                        if is_within_main_fn(cx, expr.hir_id) {
                            diag.help("`AppExit` implements `Termination`, so it can be returned directly from `fn main()`");

                            if let Some(snippet) = snippet {
                                diag.span_suggestion(
                                    expr.span,
                                    "try",
                                    format!("return {snippet}"),
                                    Applicability::MaybeIncorrect,
                                );
                            }
                        } else {
                            diag.help(
                                "Consider logging a warning if the returned `AppExit` is an error.",
                            );

                            if let Some(snippet) = snippet {
                                diag.span_suggestion(
                                    expr.span,
                                    "try",
                                    format!("let _app_exit = {snippet}"),
                                    // I believe this should always be machine applicable. We are
                                    // matching an expression, which is always allowed in the
                                    // grammar for `let` statements.
                                    // See <https://doc.rust-lang.org/reference/statements.html#let-statements>.
                                    Applicability::MachineApplicable,
                                );
                            }
                        }
                    },
                );
            }
        }
    }
}

fn is_within_main_fn<'tcx>(cx: &LateContext<'tcx>, hir_id: HirId) -> bool {
    let hir = cx.tcx.hir();

    // Find the `DefId` of the entrypoint function.
    let Some((entry_fn_id, _)) = cx.tcx.entry_fn(()) else {
        // There is no entrypoint function, so the `HirId` cannot be within it. Exit early!
        return false;
    };

    // Iterate over all parents of the `HirId`, checking if one of them is the entrypoint function.
    for (parent_id, parent) in hir.parent_iter(hir_id) {
        // If this parent is an owner (and not an expression or something else), extract it's
        // `OwnerId`. If it is not an owner, it cannot be a function, so we continue traversing up
        // the tree.
        let Some(parent_id) = parent_id.as_owner() else {
            continue;
        };

        // If the parent is `fn main()`, return true!
        if parent_id.to_def_id() == entry_fn_id {
            return true;
        }

        // If the parent is another function, but not the entrypoint, return false. We don't want to
        // emit a match for functions within functions.
        if matches!(
            parent,
            Node::Item(Item {
                kind: ItemKind::Fn(_, _, _),
                ..
            })
        ) {
            return false;
        }
    }

    // If none of the parents match the above, we are not within an entrypoint function. Return
    // false.
    false
}
