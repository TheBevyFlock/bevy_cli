//! Checks for types from `std` that have an equivalent in `bevy_platform`.
//!
//! # Motivation
//!
//! `bevy_platform` helps with platform compatibility support by providing drop in replacements for
//! the following types:
//!
//! - Arc,
//! - Barrier,
//! - BarrierWaitResult,
//! - DefaultHasher,
//! - HashMap,
//! - HashSet,
//! - Instant,
//! - LazyLock,
//! - LockResult,
//! - Mutex,
//! - MutexGuard,
//! - Once,
//! - OnceLock,
//! - OnceState,
//! - PoisonError,
//! - RandomState,
//! - RwLock,
//! - RwLockReadGuard,
//! - RwLockWriteGuard,
//! - SyncCell,
//! - SyncUnsafeCell,
//! - TryLockError,
//! - TryLockResult
//!
//!
//! # Known Issues
//!
//! This lint does not currently support checking partial imported definitions. For example:
//!
//! ```
//! use std::time;
//!
//! let now = time::Instant::now();
//! ```
//!
//! Will not emit a lint.
//!
//! # Example
//!
//! ```
//! use std::time::Instant;
//! let now = Instant::now();
//! ```
//!
//! Use instead:
//!
//! ```
//! use bevy::platform::time::Instant;
//! let now = Instant::now();
//! ```

use clippy_utils::{diagnostics::span_lint_and_sugg, is_from_proc_macro, source::snippet};
use rustc_errors::Applicability;
use rustc_hir::{
    HirId, Path, PathSegment,
    def::{DefKind, Res},
};
use rustc_lint::{LateContext, LateLintPass};

use crate::{
    declare_bevy_lint, declare_bevy_lint_pass, sym, utils::hir_parse::generic_args_snippet,
};

declare_bevy_lint! {
    pub(crate) BEVY_PLATFORM_ALTERNATIVE_EXISTS,
    super::Style,
    "Used type from the `std` that has an existing alternative from `bevy_platform`",
}

declare_bevy_lint_pass! {
    pub(crate) BevyPlatformAlternativeExists => [BEVY_PLATFORM_ALTERNATIVE_EXISTS],
}

impl<'tcx> LateLintPass<'tcx> for BevyPlatformAlternativeExists {
    fn check_path(&mut self, cx: &LateContext<'tcx>, path: &Path<'tcx>, _: HirId) {
        // Skip Resolutions that are not Structs for example: `use std::time`.
        if let Res::Def(DefKind::Struct, def_id) = path.res
            // Retrieve the first path segment, this could look like: `bevy`, `std`, `serde`.
            && let Some(first_segment) = get_first_segment(path)
            // Skip if this span originates from an external macro.
            //  Or likely originates from a proc_macro, note this should be called after
            // `in_external_macro`.
            && !path.span.in_external_macro(cx.tcx.sess.source_map())
            && !is_from_proc_macro(cx, &first_segment.ident)
            // Skip if this Definition is not originating from `std`.
            && first_segment.ident.name == sym::std
            // Get the def_id of the crate from first segment.
            && let Res::Def(DefKind::Mod,crate_def_id) = first_segment.res
            // If the first segment is not the crate root, then this type was checked when
            // importing.
            && crate_def_id.is_crate_root()
            // Get potential generic arguments.
            && let Some(generic_args) = path.segments.last().map(|s| generic_args_snippet(cx, s))
        {
            // Get the Ty of this Definition.
            let ty = cx.tcx.type_of(def_id).skip_binder();
            // Check if an alternative exists in `bevy_platform`.
            if let Some(bevy_platform_alternative) = BevyPlatformType::try_from_ty(cx, ty) {
                span_lint_and_sugg(
                    cx,
                    BEVY_PLATFORM_ALTERNATIVE_EXISTS,
                    path.span,
                    BEVY_PLATFORM_ALTERNATIVE_EXISTS.desc,
                    format!(
                        "the type `{}` can be replaced with the `no_std` compatible type {}{}",
                        snippet(cx.tcx.sess, path.span, ""),
                        bevy_platform_alternative.full_path(),
                        generic_args,
                    ),
                    format!("{}{}", bevy_platform_alternative.full_path(), generic_args),
                    Applicability::MachineApplicable,
                );
            }
        }
    }
}

/// Returns the first named segment of a [`Path`].
///
/// If this is a global path (such as `::std::fmt::Debug`), then the segment after [`kw::PathRoot`]
/// is returned.
fn get_first_segment<'tcx>(path: &Path<'tcx>) -> Option<&'tcx PathSegment<'tcx>> {
    match path.segments {
        // A global path will have PathRoot as the first segment. In this case, return the segment
        // after.
        [x, y, ..] if x.ident.name == rustc_span::symbol::kw::PathRoot => Some(y),
        [x, ..] => Some(x),
        _ => None,
    }
}

/// Creates an enum containing all the types form `bevy_platform` as variants.
///
/// # Example
///
/// ```ignore
/// declare_bevy_platform_types! {
///    // The variant name => [`PathLookup`] that matches the equivalent type in the std.
///    CustomType => CUSTOMTYPE,
///    // Optional the module path can be passed too, default is `bevy::platform::<variant_name>`.
///    // If an additional module path is added, it will result in: `bevy::platform::custom::thread::CustomType`.
///    CustomType("custom::thread") => BARRIER,
/// ```
macro_rules! declare_bevy_platform_types {
    (
    $(
        $variant:ident $(($mod_path:expr))? => $path:ident
    )
    ,+$(,)?
    ) => {
        #[derive(Copy, Clone)]
        pub enum BevyPlatformType {
            $(
                $variant,
            )+
        }

        impl BevyPlatformType{
            /// Try to create a [`BevyPlatformType`] from the given [`Ty`].
            pub fn try_from_ty(cx: &LateContext<'_>, ty: rustc_middle::ty::Ty<'_>) -> Option<Self> {
                use crate::paths::bevy_platform_types::*;
                $(
                    if $path.matches_ty(cx, ty) {
                        Some(Self::$variant)
                    } else
                )+
                {
                    None
                }
            }

            ///Returns a string identifying this [`BevyPlatformType`]. This string is suitable for user output.
            pub const fn full_path(&self) -> &'static str {
                match self {
                    $(Self::$variant => concat!("bevy::platform", $("::", $mod_path,)? "::" , stringify!($variant)),)+
                }
            }
        }
    };
}

declare_bevy_platform_types! {
    Arc("sync") => ARC,
    Barrier("sync") => BARRIER,
    BarrierWaitResult("sync") => BARRIERWAITRESULT,
    DefaultHasher("hash") => DEFAULTHASHER,
    HashMap("collections") => HASHMAP,
    HashSet("collections") => HASHSET,
    Instant("time") => INSTANT,
    LazyLock("sync") => LAZYLOCK,
    LockResult("sync") => LOCKRESULT,
    Mutex("sync") => MUTEX,
    MutexGuard("sync") => MUTEXGUARD,
    Once("sync")=> ONCE,
    OnceLock("sync") => ONCELOCK,
    OnceState("sync") => ONCESTATE,
    PoisonError("sync") => POISONERROR,
    RandomState("hash") => RANDOMSTATE,
    RwLock("sync") => RWLOCK,
    RwLockReadGuard("sync") => RWLOCKREADGUARD,
    RwLockWriteGuard("sync") => RWLOCKWRITEGUARD,
    SyncCell("sync") => SYNCCELL,
    SyncUnsafeCell("cell") => SYNCUNSAFECELL,
    TryLockError("sync") => TRYLOCKERROR,
    TryLockResult("sync") => TRYLOCKRESULT,
}
