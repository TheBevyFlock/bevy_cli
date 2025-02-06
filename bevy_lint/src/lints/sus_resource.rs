use crate::{
    declare_bevy_lint, declare_bevy_lint_pass,
    utils::hir_parse::{detuple, generic_type_at},
};
use clippy_utils::{
    diagnostics::span_lint_and_help,
    ty::match_type,
};
use rustc_hir_analysis::collect::ItemCtxt;
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;

declare_bevy_lint! {
    pub EVENTS_RESOURCE_SYSTEM_PARAM,
    RESTRICTION,
    // FIXME: Better note.
    "use of bad system parameter",
}

declare_bevy_lint_pass! {
    pub EventsResourceSystemParam => [EVENTS_RESOURCE_SYSTEM_PARAM.lint],
}

impl<'tcx> LateLintPass<'tcx> for EventsResourceSystemParam {
    fn check_ty(&mut self, cx: &LateContext<'tcx>, hir_ty: &'tcx rustc_hir::Ty<'tcx>) {
        let item_cx = ItemCtxt::new(cx.tcx, hir_ty.hir_id.owner.def_id);
        let ty = item_cx.lower_ty(hir_ty);

        let Some(res_kind) = ResKind::try_from_ty(cx, ty) else {
            return;
        };

        let Some(res_data_ty) = generic_type_at(cx, hir_ty, 1) else {
            return;
        };

        // FIXME: Is detuple necessary ? Or will `T` in `Res<T>` always be a single type ?
        detuple(*res_data_ty)
            .iter()
            .filter(|&hir_ty| match_type(cx, item_cx.lower_ty(hir_ty), &crate::paths::EVENTS))
            .for_each(|hir_ty| {
                span_lint_and_help(
                    cx,
                    EVENTS_RESOURCE_SYSTEM_PARAM.lint,
                    hir_ty.span,
                    EVENTS_RESOURCE_SYSTEM_PARAM.lint.desc,
                    None,
                    res_kind.help(None),
                )
            });
    }
}

enum ResKind {
    Res,
}

impl ResKind {
    fn try_from_ty<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Option<Self> {
        if match_type(cx, ty, &crate::paths::RES) {
            Some(Self::Res)
        } else {
            None
        }
    }

    fn help(&self, o_ty: Option<&Ty<'_>>) -> String {
        match self {
            Self::Res => format!("events Resource footgun"),
        }
    }
}
