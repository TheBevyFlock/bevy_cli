//! Utility functions for parsing HIR types.

use clippy_utils::{paths::PathLookup, source::snippet_opt};
use rustc_hir::{
    Expr, ExprKind, GenericArg, GenericArgs, Impl, Node, Path, PathSegment, QPath, Ty, TyKind,
    def::{DefKind, Res},
};
use rustc_lint::LateContext;
use rustc_span::{Ident, Span, kw};

/// Returns the list of types inside a tuple type.
///
/// If the type is not a tuple, returns a list containing the type itself.
///
/// This function will work for both tuples and references to tuples,
/// such as `(f32, &str)` and `&(f32, &str)`.
pub fn detuple(ty: Ty<'_>) -> Vec<Ty<'_>> {
    if let TyKind::Tup(items) = ty.peel_refs().kind {
        items.to_vec()
    } else {
        vec![ty]
    }
}

/// Gets the [`Ty`] for a generic argument at the specified index.
///
/// If the generic argument is not a type, returns `None`.
pub fn generic_type_at<'tcx>(
    cx: &LateContext<'tcx>,
    hir_ty: &'tcx Ty<'tcx>,
    index: usize,
) -> Option<&'tcx Ty<'tcx>> {
    let generic_arg = generic_at(hir_ty, index)?;
    let generic_node = cx.tcx.hir_node(generic_arg.hir_id());

    if let Node::Ty(ty) = generic_node {
        Some(ty)
    } else {
        None
    }
}
/// Returns the [`GenericArg`] at the given index.
pub fn generic_at<'hir>(hir_ty: &'hir Ty<'hir>, index: usize) -> Option<&'hir GenericArg<'hir>> {
    let TyKind::Path(QPath::Resolved(_, path)) = hir_ty.kind else {
        return None;
    };

    path.segments.last()?.args().args.get(index)
}

/// Returns the [`Span`] of an array of method arguments.
///
/// [`ExprKind::MethodCall`] does not provide a good method for extracting the [`Span`] of _just_
/// the method's arguments. Instead, it contains a [`slice`] of [`Expr`]. This function tries its
/// best to find a span that contains all arguments from the passed [`slice`].
///
/// This function assumes that `args` is sorted by order of appearance. An [`Expr`] that appears
/// earlier in the source code should appear earlier in the [`slice`].
///
/// If there are no [`Expr`]s in the [`slice`], this will return [`Span::default()`].
pub fn span_args(args: &[Expr]) -> Span {
    match args {
        [] => Span::default(),
        [single] => single.span,
        // Concatenate the spans together.
        [first, .., last] => first.span.to(last.span),
    }
}

/// Returns a code snipped of the generics in a [`PathSegment`], formatted as `::<A, B>`.
///
/// If no generics are present, an empty string is returned.
pub fn generic_args_snippet(cx: &LateContext, method_path: &PathSegment) -> String {
    method_path
        .args
        .and_then(GenericArgs::span_ext) // Find the span of the generics.
        .and_then(|span| snippet_opt(cx, span)) // Extract the string, which may look like `<A, B>`.
        .map(|snippet| format!("::{snippet}")) // Insert `::` before the string.
        .unwrap_or_default() // If any of the previous failed, return an empty string.
}

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
                    let inputs = cx.tcx.fn_arg_names(def_id);

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
                            QPath::Resolved(_, path @ Path { segments: [], .. }) => unreachable!(
                                "found a function call path with no segments at {:?}",
                                path.span
                            ),
                            // Lang items are not supported.
                            QPath::LangItem(_, _) => return None,
                        };

                        // Match the first argument as `receiver`, then group the rest into the
                        // slice `args`.
                        let [receiver, args @ ..] = args else {
                            // This can only happen if `args == &[]`, which shouldn't be possible,
                            // since we previously ensured that the the first element to `args`
                            // existed and was `self`.
                            unreachable!(
                                "arguments to function call was empty, even though `self` was expected, at {:?}",
                                expr.span
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

/// Checks if the [`Impl`] implements a given trait from Bevy.
pub fn impls_trait(cx: &LateContext, impl_: &Impl, trait_path: &PathLookup) -> bool {
    impl_.of_trait.is_some_and(|of_trait| {
        matches!(
            of_trait.path.res,
            // Is the trait being implemented the specified trait from Bevy?
            Res::Def(_, trait_def_id) if trait_path.matches(cx, trait_def_id)
        )
    })
}
