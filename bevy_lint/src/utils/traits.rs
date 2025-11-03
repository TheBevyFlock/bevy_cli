use clippy_utils::paths::PathLookup;
use rustc_hir::{HirId, Item, ItemKind, Node, OwnerId, QPath, TyKind, def::DefKind};
use rustc_lint::LateContext;
use rustc_span::Span;

/// Represents a type that implements a specific trait.
#[derive(Debug)]
pub struct TraitType {
    /// The [`HirId`] pointing to the type item declaration.
    pub hir_id: HirId,
    /// The span where the type was declared.
    pub item_span: Span,
    /// The span where the trait was implemented.
    pub impl_span: Span,
}

impl TraitType {
    pub fn from_local_crate<'tcx, 'a>(
        cx: &'a LateContext<'tcx>,
        trait_path: &'a PathLookup,
    ) -> impl Iterator<Item = Self> + use<'tcx, 'a> {
        // Find the `DefId` of the trait. There may be multiple if there are multiple versions of
        // the same crate.
        let trait_def_ids = trait_path
            .get(cx)
            .iter()
            .filter(|&def_id| cx.tcx.def_kind(def_id) == DefKind::Trait);

        // Find a map of all trait `impl` items within the current crate. The key is the `DefId` of
        // the trait, and the value is a `Vec<LocalDefId>` for all `impl` items.
        let all_trait_impls = cx.tcx.all_local_trait_impls(());

        // Find all `impl` items for the specific trait.
        let trait_impls = trait_def_ids
            .filter_map(|def_id| all_trait_impls.get(def_id))
            .flatten()
            .copied();

        // Map the `DefId`s of `impl` items to `TraitType`s. Sometimes this conversion can fail, so
        // we use `filter_map()` to skip errors.
        trait_impls.filter_map(move |local_def_id| {
            // Retrieve the node of the `impl` item from its `DefId`.
            let node = cx.tcx.hir_node_by_def_id(local_def_id);

            // Verify that it's an `impl` item and not something else.
            let Node::Item(Item {
                kind: ItemKind::Impl(impl_),
                span: impl_span,
                ..
            }) = node
            else {
                return None;
            };

            // Find where `T` in `impl T` was originally defined, after peeling away all references
            // `&`. This was adapted from `clippy_utils::path_res()` in order to avoid passing
            // `LateContext` to this function.
            let def_id = match impl_.self_ty.peel_refs().kind {
                TyKind::Path(QPath::Resolved(_, path)) => path.res.opt_def_id()?,
                _ => return None,
            };

            // Tries to convert the `DefId` to a `LocalDefId`, exiting early if it cannot be done.
            // This will only work if `T` in `impl T` is defined within the same crate.
            //
            // In most cases this will succeed due to Rust's orphan rule, but it notably fails
            // within `bevy_reflect` itself, since that crate implements `Reflect` for `std` types
            // such as `String`.
            let local_def_id = def_id.as_local()?;

            // Find the `HirId` from the `LocalDefId`. This is like a `DefId`, but with further
            // constraints on what it can represent.
            let hir_id = OwnerId {
                def_id: local_def_id,
            }
            .into();

            // Find the span where the type was declared. This is guaranteed to be an item, so we
            // can safely call `expect_item()` without it panicking.
            let item_span = cx.tcx.hir_node(hir_id).expect_item().span;

            Some(TraitType {
                hir_id,
                item_span,
                impl_span: *impl_span,
            })
        })
    }
}

/// A custom equality implementation that just checks the [`HirId`] of the [`TraitType`], and skips
/// the other values.
///
/// [`TraitType`]s with equal [`HirId`]s are guaranteed to be equal in all other fields, so this
/// takes advantage of that fact.
impl PartialEq for TraitType {
    fn eq(&self, other: &Self) -> bool {
        self.hir_id == other.hir_id
    }
}
