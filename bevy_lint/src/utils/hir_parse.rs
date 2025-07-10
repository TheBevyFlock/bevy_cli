//! Utility functions for parsing HIR types.

use clippy_utils::{paths::PathLookup, source::snippet_opt};
use rustc_hir::{
    Expr, GenericArg, GenericArgs, Impl, Node, PathSegment, QPath, Ty, TyKind, def::Res,
};
use rustc_lint::LateContext;
use rustc_span::Span;

use crate::span_unreachable;

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
/// [`ExprKind::MethodCall`](rustc_hir::ExprKind::MethodCall) does not provide a good method for
/// extracting the [`Span`] of _just_ the method's arguments. Instead, it contains a [`slice`] of
/// [`Expr`]. This function tries its best to find a span that contains all arguments from the
/// passed [`slice`].
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
