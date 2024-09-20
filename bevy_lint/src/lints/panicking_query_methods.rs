//! TODO:
//!
//! - Detect alternative method call syntax

use clippy_utils::{
    diagnostics::span_lint_and_help,
    source::{snippet, snippet_opt},
    ty::match_type,
};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::{Span, Symbol};
use std::borrow::Cow;

declare_tool_lint! {
    pub bevy::PANICKING_QUERY_METHODS,
    Warn, // TODO: Set to `Allow`.
    "called a `Query` method that can panic when a non-panicking alternative exists"
}

declare_lint_pass! {
    PanickingQueryMethods => [PANICKING_QUERY_METHODS]
}

/// A list of panicking `Query` methods and their non-panicking alternatives.
const PANICKING_ALTERNATIVES: &[(&str, &str)] = &[
    ("single", "get_single"),
    ("single_mut", "get_single_mut"),
    ("many", "get_many"),
    ("many_mut", "get_many_mut"),
];

impl<'tcx> LateLintPass<'tcx> for PanickingQueryMethods {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &Expr<'tcx>) {
        // Find a method call.
        if let ExprKind::MethodCall(path, src, args, method_span) = expr.kind {
            // Get the type of `src` for `src.method()`. We peel all references to that `Foo`,
            // `&Foo`, `&&Foo`, etc. all look identical, since method calls automatically
            // dereference the source.
            let src_ty = cx.typeck_results().expr_ty(src).peel_refs();

            // If `src` is a Bevy `Query`, exit.
            // TODO: Check for `QueryState`.
            if !match_type(cx, src_ty, &crate::paths::QUERY) {
                return;
            }

            // Here we check if the method name matches one of methods in `PANICKING_ALTERNATIVES`.
            // If it does match, we store the recommended alternative for reference in diagnostics
            // later. If nothing matches, we exit the entire function.
            let alternative = 'block: {
                for (panicking_method, alternative_method) in PANICKING_ALTERNATIVES {
                    // TODO: Intern these earlier / cache the results?
                    let key = Symbol::intern(panicking_method);

                    if path.ident.name == key {
                        // It is one of the panicking methods. Write down the alternative and stop
                        // searching.
                        break 'block *alternative_method;
                    }
                }

                // If we reach this point, the method is not one we're searching for. In this case,
                // we exit.
                return;
            };

            // By this point, we've verified that `src` is `Query` and the method is a panicking
            // one. Let's emit the lint.

            // Try to find the string representation of `src`. This usually returns `my_query`
            // without the trailing `.`, so we manually append it. When the snippet cannot be
            // found, we default to the qualified `Query::` form.
            let src_snippet: Cow<str> =
                snippet_opt(cx, src.span).map_or("Query::".into(), |mut s| {
                    s.push('.');
                    s.into()
                });

            // Try to find the string representation of the arguments to our panicking method. See
            // `span_args()` for more details on how this is done.
            let args_snippet = snippet(cx, span_args(args), "");

            span_lint_and_help(
                cx,
                PANICKING_QUERY_METHODS,
                method_span,
                PANICKING_QUERY_METHODS.desc,
                None,
                // This usually ends up looking like: `query.get_many([e1, e2])`.
                format!("use `{src_snippet}{alternative}({args_snippet})` and handle the `Result`"),
            );
        }
    }
}

/// Returns the [`Span`] of an array of method arguments.
///
/// [`ExprKind::MethodCall`] does not provide a good method for extracting the [`Span`] of _just_
/// the method's arguments. Instead, it contains a [`slice`] of [`Expr`]. This function tries it's
/// best to find a span that contains all arguments from the passed [`slice`].
///
/// This function assumes that `args` is sorted by order of appearance. An [`Expr`] that appears
/// earlier in the source code should appear earlier in the [`slice`].
///
/// If there are no [`Expr`]s in the [`slice`], this will return [`Span::default()`].
fn span_args(args: &[Expr]) -> Span {
    // Start with an empty span. If `args` is empty, this will be returned. This may look like
    // `0..0`.
    let mut span = Span::default();

    // If at least 1 item exists in `args`, get the first expression and overwrite `span` with it's
    // value. `span` may look like `7..12` now, with a bit of extra metadata.
    if let Some(first_arg) = args.first() {
        span = first_arg.span;
    }

    // Get the last `Expr`, if it exists, and overwrite our span's highest index with the last
    // expression's highest index. If there is only one item in `args`, this will appear to do
    // nothing. `span` may now look like `7..20`.
    if let Some(last_arg) = args.last() {
        span = span.with_hi(last_arg.span.hi());
    }

    span
}
