use clippy_utils::{diagnostics::span_lint_hir_and_then, sugg::DiagExt, ty::implements_trait};
use rustc_errors::Applicability;
use rustc_hir::ItemKind;
use rustc_lint::{LateContext, LateLintPass};

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::traits::TraitType};

declare_bevy_lint! {
    //TODO: include unit struct here, since this lint only applies to those.
    pub(crate) MISSING_DEFAULT,
    super::Restriction,
    "defined a unit componentwithout a `Default` implementation",
    @crate_level_only = true,
}

declare_bevy_lint_pass! {
    pub(crate) MissingDefault => [MISSING_DEFAULT],
}

impl<'tcx> LateLintPass<'tcx> for MissingDefault {
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        // Finds all types that implement `Component` in this crate.
        let components: Vec<TraitType> =
            TraitType::from_local_crate(cx, &crate::paths::COMPONENT).collect();

        // Lookup the `DefId` of the `Default` trait
        let Some(default_trait_def_id) =
            cx.tcx.get_diagnostic_item(rustc_span::symbol::sym::Default)
        else {
            return;
        };

        for component in components {
            let def_id = component.hir_id.owner.to_def_id();

            // Skip binder as we are not interested in the generics
            let ty = cx.tcx.type_of(def_id).skip_binder();

            // `Default` is implemented
            if implements_trait(cx, ty, default_trait_def_id, &[]) {
                continue;
            }

            // Find the `Item` definition of the Component missing `#[derive(Default)]`.
            let without_default_item = cx
                .tcx
                .hir_expect_item(component.hir_id.expect_owner().def_id);

            let fields = match without_default_item.kind {
                ItemKind::Struct(_, _, data) => data.fields().to_vec(),
                // If this item is not a struct, continue
                _ => continue,
            };

            // Check if the struct is a unit struct (contains no fields)
            if !fields.is_empty() {
                return;
            }

            span_lint_hir_and_then(
                cx,
                MISSING_DEFAULT,
                // This tells `rustc` where to search for `#[allow(...)]` attributes.
                component.hir_id,
                component.item_span,
                "defined a unit struct without a `Default` implementation",
                |diag| {
                    diag.span_note(component.impl_span, "`Component` implemented here")
                        .suggest_item_with_attr(
                            cx,
                            component.item_span,
                            "`Default` can be automatically derived",
                            "#[derive(Default)]",
                            // This lint is MachineApplicable, since we only lint structs that do
                            // not have any fields.
                            // This suggestion may result in two consecutive
                            // `#[derive(...)]` attributes, but `rustfmt` merges them
                            // afterwards.
                            Applicability::MachineApplicable,
                        );
                },
            );
        }
    }
}
