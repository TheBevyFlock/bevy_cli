//! Checks for unit structs that do not  implement `Copy`, `Clone` or `Default`.
//!
//! # Motivation
//!
//! This is mainly useful for the Bevy engine itself to ensure a consistent API. For example
//! in order to use a component with required components the component needs to implement
//! `Default`.
//!
//! # Example
//!
//! ```
//! struct UnitStruct;
//! ```
//!
//! Use instead:
//!
//! ```
//! #[derive(Copy,Clone,Default)]
//! struct MyComponent;
//! ```
use clippy_utils::{diagnostics::span_lint_and_then, sugg::DiagExt, ty::implements_trait};
use rustc_errors::Applicability;
use rustc_hir::{Item, ItemKind, VariantData, def_id::DefId};
use rustc_lint::{LateContext, LateLintPass, Lint};

use crate::{declare_bevy_lint, declare_bevy_lint_pass};

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
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &Item) {
        // Skip if this Item originates from a foreign crate's macro
        if item.span.in_external_macro(cx.tcx.sess.source_map()) {
            return;
        }

        // Check that the item is a Struct.
        let ItemKind::Struct(_, _, data) = item.kind else {
            return;
        };

        // Check if the struct is a unit struct (contains no fields).
        if !matches!(data, VariantData::Unit(..)) {
            return;
        }

        // Skip binder as unit types cannot have generics.
        let ty = cx.tcx.type_of(item.owner_id.to_def_id()).skip_binder();

        for trait_to_implement in Trait::all() {
            // get the def_id of the trait that should be implement for unit structures.
            if let Some(trait_def_id) = trait_to_implement.get_def_id(cx)
                    // Unit types cannot have generic arguments, so we don't need to pass any in.
                    && !implements_trait(cx, ty, trait_def_id, &[])
            {
                span_lint_and_then(
                    cx,
                    trait_to_implement.lint(),
                    // This tells `rustc` where to search for `#[allow(...)]` attributes.
                    item.span,
                    format!(
                        "defined a unit struct without a `{}` implementation",
                        trait_to_implement.name()
                    ),
                    |diag| {
                        diag.suggest_item_with_attr(
                            cx,
                            item.span,
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
