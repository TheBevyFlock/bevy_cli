use clippy_utils::{diagnostics::span_lint_hir_and_then, sugg::DiagExt, ty::implements_trait};
use rustc_errors::Applicability;
use rustc_hir::{ItemKind, def_id::DefId};
use rustc_lint::{LateContext, LateLintPass, Lint};

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::traits::TraitType};

declare_bevy_lint! {
    pub(crate) MISSING_DEFAULT,
    super::Restriction,
    "defined a unit component without a `Default` implementation",
}

declare_bevy_lint! {
    pub(crate) MISSING_CLONE,
    super::Restriction,
    "defined a unit component without a `Clone` implementation",
}

declare_bevy_lint! {
    pub(crate) MISSING_COPY,
    super::Restriction,
    "defined a unit component without a `Copy` implementation",
}

declare_bevy_lint_pass! {
    pub(crate) MissingTraitForUnitStruct => [MISSING_DEFAULT,MISSING_CLONE,MISSING_COPY],
}

impl<'tcx> LateLintPass<'tcx> for MissingTraitForUnitStruct {
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        // Finds all types that implement `Component` in this crate.
        let components = TraitType::from_local_crate(cx, &crate::paths::COMPONENT);

        for component in components {
            // Skip if a types originates from a foreign crate's macro
            if component
                .item_span
                .in_external_macro(cx.tcx.sess.source_map())
            {
                continue;
            }

            let def_id = component.hir_id.expect_owner().to_def_id();

            // Skip binder as unit types cannot have generics.
            let ty = cx.tcx.type_of(def_id).skip_binder();

            for trait_to_implement in Trait::all() {
                // get the def_id of the trait that should be implement for unit structures.
                if let Some(trait_def_id) = trait_to_implement.get_def_id(cx)
                    // Unit types cannot have generic arguments, so we don't need to pass any in.
                    && !implements_trait(cx, ty, trait_def_id, &[])
                {
                    // Find the `Item` definition of the Component missing the trait derive.
                    let missing_trait_imp_item = cx
                        .tcx
                        .hir_expect_item(component.hir_id.expect_owner().def_id);

                    let fields = match missing_trait_imp_item.kind {
                        ItemKind::Struct(_, _, data) => data.fields().to_vec(),
                        // If this item is not a struct, continue
                        _ => continue,
                    };

                    // Check if the struct is a unit struct (contains no fields)
                    if !fields.is_empty() {
                        continue;
                    }

                    span_lint_hir_and_then(
                        cx,
                        trait_to_implement.lint(),
                        // This tells `rustc` where to search for `#[allow(...)]` attributes.
                        component.hir_id,
                        component.item_span,
                        format!(
                            "defined a unit struct without a `{}` implementation",
                            trait_to_implement.name()
                        ),
                        |diag| {
                            diag.span_note(component.impl_span, "`Component` implemented here")
                                .suggest_item_with_attr(
                                    cx,
                                    component.item_span,
                                    format!(
                                        "`{}` can be automatically derived",
                                        trait_to_implement.name()
                                    )
                                    .as_str(),
                                    format!("#[derive({})]", trait_to_implement.name()).as_str(),
                                    // This lint is MachineApplicable, since we only lint structs
                                    // that do not have any
                                    // fields. This suggestion
                                    // may result in two consecutive
                                    // `#[derive(...)]` attributes, but `rustfmt` merges them
                                    // afterwards.
                                    Applicability::MachineApplicable,
                                );
                        },
                    );
                }
            }
        }
    }
}

enum Trait {
    Copy,
    Clone,
    Default,
}

impl Trait {
    const fn all() -> [Trait; 3] {
        use Trait::*;

        [Copy, Clone, Default]
    }

    const fn name(&self) -> &'static str {
        match self {
            Trait::Copy => "Copy",
            Trait::Clone => "Clone",
            Trait::Default => "Default",
        }
    }

    const fn lint(&self) -> &'static Lint {
        match self {
            Trait::Copy => MISSING_COPY,
            Trait::Clone => MISSING_CLONE,
            Trait::Default => MISSING_DEFAULT,
        }
    }

    fn get_def_id(&self, cx: &LateContext) -> std::option::Option<DefId> {
        match self {
            Trait::Copy => cx.tcx.get_diagnostic_item(rustc_span::symbol::sym::Copy),
            Trait::Clone => cx.tcx.get_diagnostic_item(rustc_span::symbol::sym::Clone),
            Trait::Default => cx.tcx.get_diagnostic_item(rustc_span::symbol::sym::Default),
        }
    }
}
