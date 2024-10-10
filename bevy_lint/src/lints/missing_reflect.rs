//! TODO

use crate::declare_bevy_lint;
use clippy_utils::def_path_res;
use rustc_hir::{
    def::{DefKind, Res},
    def_id::{DefId, LocalDefId},
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

impl<'tcx> LateLintPass<'tcx> for MissingReflect {
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        const CHECKED_TRAITS: [&[&str]; 2] = [&crate::paths::COMPONENT, &crate::paths::RESOURCE];

        // TODO: Convert from `impl` DID to `struct` DID.
        let reflect_impls = find_local_trait_impls(cx.tcx, &crate::paths::REFLECT);

        println!("REFLECT: {reflect_impls:?}");

        for trait_def_path in CHECKED_TRAITS {
            // TODO: Convert from `impl` DID to `struct` DID.
            let trait_impls = find_local_trait_impls(cx.tcx, trait_def_path);

            for impl_ in trait_impls {
                if !reflect_impls.contains(&impl_) {
                    println!("Found non reflect {trait_def_path:?}: {impl_:?}");
                }
            }
        }
    }
}

fn find_local_trait_impls(tcx: TyCtxt<'_>, trait_def_path: &[&str]) -> Vec<LocalDefId> {
    let trait_def_ids = find_trait_did_for_def_path(tcx, trait_def_path);

    if trait_def_ids.is_empty() {
        return Vec::new();
    }

    // TODO: Warn / debug only?
    if trait_def_ids.len() > 1 {
        println!("Multiple `DefIds` found for {trait_def_path:?}: {trait_def_ids:?}");
    }

    let local_trait_impls = tcx.all_local_trait_impls(());

    trait_def_ids
        .into_iter()
        .filter_map(|trait_def_id| local_trait_impls.get(&trait_def_id))
        .flatten()
        .copied()
        .collect()
}

fn find_trait_did_for_def_path(tcx: TyCtxt<'_>, trait_def_path: &[&str]) -> Vec<DefId> {
    def_path_res(tcx, trait_def_path)
        .into_iter()
        .filter_map(|res| match res {
            Res::Def(DefKind::Trait, def_id) => Some(def_id),
            _ => None,
        })
        .collect()
}
