//! TODO

use crate::declare_bevy_lint;
use clippy_utils::{def_path_res, diagnostics::span_lint_hir_and_then, sugg::DiagExt};
use rustc_errors::Applicability;
use rustc_hir::{
    def::{DefKind, Res},
    HirId, Item, ItemKind, Node, OwnerId, QPath, TyKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyCtxt;
use rustc_session::declare_lint_pass;
use rustc_span::Span;

declare_bevy_lint! {
    pub MISSING_REFLECT,
    RESTRICTION,
    "defined a component, resource, or event without a `Reflect` implementation",
}

declare_lint_pass! {
    MissingReflect => [MISSING_REFLECT.lint]
}

impl<'tcx> LateLintPass<'tcx> for MissingReflect {
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        // Finds all types that implement `Reflect` in this crate.
        let reflected: Vec<TraitType> =
            TraitType::from_local_crate(cx.tcx, &crate::paths::REFLECT).collect();

        // Finds all non-`Reflect` types that implement `Event` in this crate.
        let events: Vec<TraitType> = TraitType::from_local_crate(cx.tcx, &crate::paths::EVENT)
            .filter(|trait_type| !reflected.contains(trait_type))
            .collect();

        // Finds all non-`Reflect` types that implement `Component` and *not* `Event` in this
        // crate. Because events are also components, we need to deduplicate the two to avoid
        // emitting multiple diagnostics for the same type.
        let components: Vec<TraitType> =
            TraitType::from_local_crate(cx.tcx, &crate::paths::COMPONENT)
                .filter(|trait_type| {
                    !(reflected.contains(trait_type) || events.contains(trait_type))
                })
                .collect();

        // Finds all non-`Reflect` types that implement `Resource` in this crate.
        let resources: Vec<TraitType> =
            TraitType::from_local_crate(cx.tcx, &crate::paths::RESOURCE)
                .filter(|trait_type| !reflected.contains(trait_type))
                .collect();

        for (checked_trait, trait_name) in [
            (events, "Event"),
            (components, "Component"),
            (resources, "Resource"),
        ] {
            for without_reflect in checked_trait {
                span_lint_hir_and_then(
                    cx,
                    MISSING_REFLECT.lint,
                    without_reflect.hir_id,
                    without_reflect.item_span,
                    MISSING_REFLECT.lint.desc,
                    |diag| {
                        diag.span_note(
                            without_reflect.impl_span,
                            format!("`{trait_name}` implemented here"),
                        )
                        .suggest_item_with_attr(
                            cx,
                            without_reflect.item_span,
                            "`Reflect` can be automatically derived",
                            "#[derive(Reflect)]",
                            Applicability::MachineApplicable,
                        );
                    },
                );
            }
        }
    }
}

/// Represents a type that implements a specific trait.
#[derive(Debug)]
struct TraitType {
    hir_id: HirId,
    item_span: Span,
    impl_span: Span,
}

impl TraitType {
    fn from_local_crate<'tcx>(
        tcx: TyCtxt<'tcx>,
        trait_path: &[&str],
    ) -> impl Iterator<Item = Self> + 'tcx {
        // Find the `DefId` of the trait. There may be multiple if there are multiple versions of
        // the same crate.
        let trait_def_ids = def_path_res(tcx, trait_path)
            .into_iter()
            .filter_map(|res| match res {
                Res::Def(DefKind::Trait, def_id) => Some(def_id),
                _ => None,
            });

        // Find a map of all trait `impl` items within the current crate. The key is the `DefId` of
        // the trait, and the value is a `Vec<LocalDefId>` for all `impl` items.
        let all_trait_impls = tcx.all_local_trait_impls(());

        // Find all `impl` items for the specific trait.
        let trait_impls = trait_def_ids
            .filter_map(|def_id| all_trait_impls.get(&def_id))
            .flatten()
            .copied();

        // Map the `DefId`s of `impl` items to `TraitType`s. Sometimes this conversion can fail, so
        // we use `filter_map()` to skip errors.
        trait_impls.filter_map(move |local_def_id| {
            // Retrieve the node of the `impl` item from its `DefId`.
            let node = tcx.hir_node_by_def_id(local_def_id);

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

            let hir_id = OwnerId {
                // This is guaranteed to be a `LocalDefId` due to Rust's orphan rule. The traits
                // (`Reflect`, `Component`, etc.) are from an external crate, so the type
                // definition _must_ be local. The only case this may not be upheld is within
                // Bevy's own crates.
                def_id: def_id.expect_local(),
            }
            .into();

            let item_span = tcx.hir_node(hir_id).expect_item().span;

            Some(TraitType {
                hir_id,
                item_span,
                impl_span: *impl_span,
            })
        })
    }
}

/// A custom equality implementation that just checks the [`DefId`] of the [`TraitType`], and skips
/// the other values.
///
/// [`TraitType`]s with equal [`DefId`]s are guaranteed to be equal in all other fields, so this
/// takes advantage of that fact.
impl PartialEq for TraitType {
    fn eq(&self, other: &Self) -> bool {
        self.hir_id == other.hir_id
    }
}
