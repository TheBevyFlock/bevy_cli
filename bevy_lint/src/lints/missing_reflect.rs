//! TODO

use crate::declare_bevy_lint;
use clippy_utils::{def_path_res, diagnostics::span_lint_hir};
use rustc_hir::{
    def::{DefKind, Res},
    def_id::{DefId, LocalDefId},
    Item, ItemKind, Node, OwnerId,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyCtxt;
use rustc_session::declare_lint_pass;

declare_bevy_lint! {
    pub MISSING_REFLECT,
    RESTRICTION,
    "defined a component, resource, or event without a `Reflect` implementation",
}

declare_lint_pass! {
    MissingReflect => [MISSING_REFLECT.lint]
}

/// A list of traits that are checked for `Reflect` implementations.
///
/// If a struct implements one of these traits but not `Reflect`, this lint will raise a warning.
const CHECKED_TRAITS: [&[&str]; 2] = [&crate::paths::COMPONENT, &crate::paths::RESOURCE];

impl<'tcx> LateLintPass<'tcx> for MissingReflect {
    // The lint can be summarized in a few steps:
    //
    // 1. Find all `impl` items for `Reflect`, `Component`, and `Resource`.
    // 2. Find the types that these traits are implemented for.
    // 3. If there's a type that implements `Component` or `Resource` but *not* `Reflect`, emit a
    //    diagnostic.
    //
    // Because we need a list of all `impl` items, not just one at a time, we implement
    // `check_crate()` and not `check_item()`.
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        // Find all `impl` items for `Reflect` in the current crate, then from that find `T` in
        // `impl Reflect for T`.
        let reflect_types: Vec<_> = find_local_trait_impls(cx.tcx, &crate::paths::REFLECT)
            .filter_map(|did| impl_to_source_type(cx.tcx, did))
            .collect();

        for trait_def_path in CHECKED_TRAITS {
            // This is the same as the above `reflect_types`, but this time we are searching for
            // one of the checked traits. (`Component` or `Resource`.)
            let checked_types: Vec<_> = find_local_trait_impls(cx.tcx, trait_def_path)
                .filter_map(|did| impl_to_source_type(cx.tcx, did))
                .collect();

            // Check if any of the checked types do not implement `Reflect`. If so, emit the lint!
            for impl_ in checked_types {
                if !reflect_types.contains(&impl_) {
                    let ident = cx.tcx.opt_item_ident(impl_).unwrap();

                    let owner_id = OwnerId {
                        // This is guaranteed to be a `LocalDefId` because the trait `impl` that it
                        // came from is also local.
                        def_id: impl_.expect_local(),
                    };

                    span_lint_hir(
                        cx,
                        MISSING_REFLECT.lint,
                        owner_id.into(),
                        ident.span,
                        MISSING_REFLECT.lint.desc,
                    );
                }
            }
        }
    }
}

/// Returns a list of [`LocalDefId`]s for `impl` blocks where a specified trait is implemented.
///
/// Note that sometimes multiple traits can be resolved from the same path. (If there are multiple
/// versions of the same crate, for example.) When this is the case, the results for each trait are
/// concatenated together.
fn find_local_trait_impls<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_def_path: &[&str],
) -> impl Iterator<Item = LocalDefId> + 'tcx {
    // Find the `DefId`s for a given trait.
    let trait_def_ids = trait_def_ids(tcx, trait_def_path);

    // Find a map of all trait `impl` items within the current crate. The key is the `DefId` of the
    // trait, and the value is a `Vec<LocalDefId>` for all `impl` items.
    let local_trait_impls = tcx.all_local_trait_impls(());

    trait_def_ids
        .filter_map(|trait_def_id| local_trait_impls.get(&trait_def_id))
        .flatten()
        .copied()
}

/// Finds all [`DefId`]s for a given trait path.
///
/// This returns an interator because multiple items can have the same name and path, such as
/// traits and macros, and because there may be multiple versions of the same crate. The returned
/// [`DefId`]s are guaranteed to point to traits, however, with all others skipped.
fn trait_def_ids(tcx: TyCtxt<'_>, trait_def_path: &[&str]) -> impl Iterator<Item = DefId> {
    def_path_res(tcx, trait_def_path)
        .into_iter()
        .filter_map(|res| match res {
            Res::Def(DefKind::Trait, def_id) => Some(def_id),
            _ => None,
        })
}

/// This function locates the source structure definition for a `impl` item [`LocalDefId`].
///
/// Given the following code:
///
/// ```
/// struct Foo; // ID: A
///
/// impl Foo {} // ID: B
/// ```
///
/// This function will return a [`DefId`] of `A` when passed the [`LocalDefId`] `B`. It even works
/// when the `impl` block implements a trait, such as `impl Bar for Foo`.
///
/// This function will return [`None`] if `def_id` does not correspond to an `impl` item, or if the
/// type `T` in `impl T` is not an ADT[^0]. References are automatically peeled, so only the
/// underlying type determines the result.
///
/// [^0]: An algebraic data type. These are most user-defined types, such as structs, enums, and
/// unions. Notably, primitives are not ADTs. See [`TyKind`](rustc_middle::ty::TyKind) for a
/// complete list.
fn impl_to_source_type(tcx: TyCtxt<'_>, def_id: LocalDefId) -> Option<DefId> {
    let node = tcx.hir_node_by_def_id(def_id);

    // Ensure the node is an `impl` item.
    let Node::Item(Item {
        kind: ItemKind::Impl(impl_),
        ..
    }) = node
    else {
        return None;
    };

    // This is the HIR representation of `T` for `impl T`. Note that the HIR representation does
    // not contain actual type information, just the qualified path.
    let hir_ty = impl_.self_ty;

    // Convert the `rustc_hir::Ty` to a `rustc_middle::ty::Ty`, which is fully resolved with
    // complete type information. Note that we can safely call `skip_binder()` because we are
    // purely extrating the type's `DefId`, which does not depend on generic or lifetime data. Also
    // note the call to `peel_refs()`, which removes references and returns the underlying type.
    let ty_adt = tcx
        .type_of(hir_ty.hir_id.owner)
        .skip_binder()
        .peel_refs()
        .ty_adt_def()?;

    Some(ty_adt.did())
}
