//! The [`MethodCall`] utility for parsing all forms of method calls.

use rustc_hir::{
    Expr, ExprKind, Path, PathSegment, QPath,
    def::{DefKind, Res},
};
use rustc_lint::LateContext;
use rustc_span::{Ident, Span, kw};

use crate::span_unreachable;

/// An abstraction over method calls that supports both `receiver.method(args)` and
/// `Struct::method(&receiver, args)`.
///
/// # Examples
///
/// ```ignore
/// fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
///     // Don't use this, it doesn't match qualified method calls!
///     if let ExprKind::MethodCall(_, _, _, span) = expr.kind {
///         // ...
///     }
///
///     // Instead use:
///     if let Some(MethodCall { span, .. }) = MethodCall::try_from(cx, expr) {
///         // ...
///     }
/// }
/// ```
///
/// # Limitations
///
/// This does not support qualified method calls where the function is not a path. For example:
///
/// ```
/// struct Foo;
///
/// impl Foo {
///     fn bar(&self) {}
/// }
///
/// // A closure that returns a function.
/// let closure_closure = || Foo::bar;
///
/// // This will not be matched, because `closure_closure()` is an `ExprKind::Call` instead of an
/// // `ExprKind::Path`.
/// (closure_closure())(&Foo);
///
/// // This *will* be matched, though, because `Foo::bar` is an `ExprKind::Path`.
/// Foo::bar(&Foo);
/// ```
///
/// Furthermore, this does not support [language items]. If [`Self::try_from()`] encounters a
/// qualified method call that is a lang item, it will still return [`None`].
///
/// [language items]: https://rustc-dev-guide.rust-lang.org/lang-items.html
#[derive(Debug)]
pub struct MethodCall<'tcx> {
    /// The path to the method.
    ///
    /// This can be used to find the name of the method, its [`DefId`](rustc_hir::def_id::DefId),
    /// and its generic arguments.
    ///
    /// # Example
    ///
    /// ```ignore
    /// receiver.method(args);
    /// //       ^^^^^^
    ///
    /// Struct::method(&receiver, args);
    /// //      ^^^^^^
    /// ```
    pub method_path: &'tcx PathSegment<'tcx>,

    /// The receiver, or the object, of the method.
    ///
    /// This can be used to find what type the method is implemented for. Note that this will
    /// include the reference in the type _only_ if the method is fully-qualified. This reference
    /// will be omitted when the method is in receiver form. As such, you may want to call
    /// [`Ty::peel_refs()`](rustc_middle::ty::Ty::peel_refs) on the result before processing it.
    ///
    /// # Example
    ///
    /// ```ignore
    /// receiver.method(args);
    /// //^^^^^^
    ///
    /// Struct::method(&receiver, args);
    /// //             ^^^^^^^^^
    /// ```
    pub receiver: &'tcx Expr<'tcx>,

    /// The arguments passed to the method.
    ///
    /// # Example
    ///
    /// ```ignore
    /// receiver.method(args);
    /// //              ^^^^
    ///
    /// Struct::method(&receiver, args);
    /// //                        ^^^^
    /// ```
    pub args: &'tcx [Expr<'tcx>],

    /// The span of the method and its arguments.
    ///
    /// This will not include the receiver if this is not a qualified method call.
    ///
    /// # Example
    ///
    /// ```ignore
    /// receiver.method(args);
    /// //       ^^^^^^^^^^^^
    ///
    /// Struct::method(&receiver, args);
    /// //^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    /// ```
    pub span: Span,

    /// Marks if this method call is fully qualified or not.
    ///
    /// This lets lints customize their suggestions to use either the receiver-based or
    /// fully-qualified forms of a method.
    pub is_fully_qulified: bool,
}

impl<'tcx> MethodCall<'tcx> {
    /// Tries to return a [`MethodCall`] from an [`Expr`].
    ///
    /// Please see the [structure documentation](MethodCall) for examples and limitations.
    pub fn try_from(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> Option<Self> {
        match expr.kind {
            ExprKind::MethodCall(method_path, receiver, args, span) => Some(Self {
                method_path,
                receiver,
                args,
                span,
                is_fully_qulified: false,
            }),
            ExprKind::Call(
                // We only want function calls where the function is a path, so we can use
                // `LateContext::qpath_res()`. This elimantes code where the function is the result
                // of another function, such as:
                //
                // ```
                // let closure_closure = || || {};
                //
                // // This entire expression will not be matched, though the inner
                // // `closure_closure()` will because `closure_closure` is a path.
                // (closure_closure())();
                // ```
                path @ Expr {
                    kind: ExprKind::Path(qpath),
                    ..
                },
                args,
            ) => {
                // Resolve the path, filtering out any paths that are not to associated functions.
                // This eliminates prevents us from matching standalone functions, such as:
                //
                // ```
                // fn function() {}
                //
                // // This will not be matched, since `function()`'s definition is not within an
                // `impl` block.
                // function();
                // ```
                if let Res::Def(DefKind::AssocFn, def_id) = cx.qpath_res(qpath, path.hir_id) {
                    // Retrieve the identifiers for all the arguments to this function.
                    let inputs = cx.tcx.fn_arg_idents(def_id);

                    // If the name of the first argument is `self`, then it *must* be a method.
                    // `self` is a reserved keyword, and cannot be used as a general function
                    // argument name.
                    if let [
                        Some(Ident {
                            name: kw::SelfLower,
                            ..
                        }),
                        ..,
                    ] = inputs
                    {
                        let method_path = match *qpath {
                            // If the qualified path is resolved, the method path must be the final
                            // segment.
                            QPath::Resolved(
                                _,
                                Path {
                                    // Match the final segment as the method path.
                                    segments: [.., method_path],
                                    ..
                                },
                            )
                            | QPath::TypeRelative(_, method_path) => method_path,
                            QPath::Resolved(_, path @ Path { segments: [], .. }) => {
                                span_unreachable!(
                                    path.span,
                                    "found a function call path with no segments",
                                )
                            }
                            // Lang items are not supported.
                            QPath::LangItem(_, _) => return None,
                        };

                        // Match the first argument as `receiver`, then group the rest into the
                        // slice `args`.
                        let [receiver, args @ ..] = args else {
                            // This can only happen if `args == &[]`, which shouldn't be possible,
                            // since we previously ensured that the the first element to `args`
                            // existed and was `self`.
                            span_unreachable!(
                                expr.span,
                                "arguments to function call was empty, even though `self` was expected",
                            );
                        };

                        return Some(Self {
                            method_path,
                            receiver,
                            args,
                            span: expr.span,
                            is_fully_qulified: true,
                        });
                    }
                }

                // If any of the above checks fail, return `None`, as it's not a qualified method
                // call.
                None
            }
            _ => None,
        }
    }
}
