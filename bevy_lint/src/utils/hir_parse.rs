//! Utility functions for parsing HIR types.

use rustc_hir::{GenericArg, Node, QPath, Ty, TyKind};
use rustc_lint::LateContext;

/// Returns the list of types inside a tuple type.
///
/// If the type is not a tuple, returns a list containing the type itself.
pub(crate) fn detuple(ty: Ty<'_>) -> Vec<Ty<'_>> {
    if let TyKind::Tup(items) = ty.peel_refs().kind {
        items.to_vec()
    } else {
        vec![ty]
    }
}

/// Gets the [`Ty`] for a generic argument at the specified index.
///
/// If the generic argument is not a type, returns `None`.
pub(crate) fn generic_type_at<'tcx>(
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
pub(crate) fn generic_at<'hir>(
    hir_ty: &'hir Ty<'hir>,
    index: usize,
) -> Option<&'hir GenericArg<'hir>> {
    let TyKind::Path(QPath::Resolved(_, path)) = hir_ty.kind else {
        return None;
    };

    path.segments.last()?.args().args.get(index)
}
