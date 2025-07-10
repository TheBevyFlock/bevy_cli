//! Panicking macros that display better error messages than the [`std`] variants.
//!
//! This internally relies on [`rustc_middle::util::bug::bug_fmt()`] and
//! [`rustc_middle::util::bug::span_bug_fmt()`], which use `rustc`'s built-in diagnostics to emit
//! the bug. This mechanism is not available in unit tests, in which case the standard library's
//! versions should be used instead. This module is not meant to be imported directly; all macros
//! are available in the crate root.
//!
//! The following macros are currently unimplemented, as they are not used by the linter:
//!
//! - `assert_ne!`
//! - `debug_assert_ne!`
//! - `unimplemented!`
//!
//! The following [`Span`](rustc_span::Span) macros are currently unimplemented as well:
//!
//! - `span_assert!`
//! - `span_assert_eq!`
//! - `debug_span_assert!`
//! - `debug_span_assert_eq!`
//!
//! [`todo!`] is purposefully unimplemented, as it should never be used in a user-facing manner.

/// A variant of [`std::panic!`] with better error messages.
#[macro_export]
macro_rules! panic {
    () => {
        $crate::panic!("explicit panic")
    };
    ($($arg:tt)+) => {
        ::rustc_middle::util::bug::bug_fmt(::std::format_args!($($arg)+))
    };
}

/// A variant of [`std::panic!`] with better error messages emitted to a specific
/// [`Span`](rustc_span::Span).
#[macro_export]
macro_rules! span_panic {
    ($span:expr) => {
        $crate::span_panic!($span, "explicit panic")
    };
    ($span:expr, $($arg:tt)+) => {
        ::rustc_middle::util::bug::span_bug_fmt($span, ::std::format_args!($($arg)+))
    };
}

/// A variant of [`std::assert!`] with better error messages.
#[macro_export]
macro_rules! assert {
    ($cond:expr $(,)?) => {
        $crate::assert!($cond, stringify!($cond));
    };

    ($cond:expr, $($arg:tt)+) => {
        if !($cond) {
            match ::std::format_args!($($arg)+) {
                message => {
                    $crate::panic!("assertion failed: {message}");
                },
            };
        }
    };
}

/// A variant of [`std::assert!`] with better error messages emitted to a specific
/// [`Span`](rustc_span::Span).
#[macro_export]
macro_rules! span_assert {
    ($span:expr, $cond:expr $(,)?) => {
        $crate::span_assert!($span, $cond, stringify!($cond));
    };

    ($span:expr, $cond:expr, $($arg:tt)+) => {
        if !($cond) {
            match ::std::format_args!($($arg)+) {
                message => {
                    $crate::span_panic!($span, "assertion failed: {message}");
                },
            };
        }
    };
}

/// A variant of [`std::assert_eq!`] with better error messages.
#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left, right) => {
                if !(*left == *right) {
                    $crate::panic!(
                        "\
assertion `left == right` failed
  left: {left:?}
 right: {right:?}",
                    );
                }
            },
        }
    };

    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left, right) => {
                if !(*left == *right) {
                    match ::std::format_args!($($arg)+) {
                        message => {
                            $crate::panic!(
                                "\
assertion `left == right` failed: {message}
  left: {left:?}
 right: {right:?}",
                            );
                        },
                    };
                }
            },
        }
    };
}

/// A variant of [`std::assert_eq!`] with better error messages emitted to a specific
/// [`Span`](rustc_span::Span).
#[macro_export]
macro_rules! span_assert_eq {
    ($span:expr, $left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left, right) => {
                if !(*left == *right) {
                    $crate::span_panic!(
                        $span,
                        "\
assertion `left == right` failed
  left: {left:?}
 right: {right:?}",
                    );
                }
            },
        }
    };

    ($span:expr, $left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left, right) => {
                if !(*left == *right) {
                    match ::std::format_args!($($arg)+) {
                        message => {
                            $crate::span_panic!(
                                $span,
                                "\
assertion `left == right` failed: {message}
  left: {left:?}
 right: {right:?}",
                            );
                        },
                    };
                }
            },
        }
    };
}

/// A variant of [`std::debug_assert!`] with better error messages.
#[macro_export]
macro_rules! debug_assert {
    ($($arg:tt)*) => {
        if ::std::cfg!(debug_assertions) {
            $crate::assert!($($arg)*);
        }
    };
}

/// A variant of [`std::debug_assert!`] with better error messages emitted to a specific
/// [`Span`](rustc_span::Span).
#[macro_export]
macro_rules! debug_span_assert {
    ($span:expr, $($arg:tt)*) => {
        if ::std::cfg!(debug_assertions) {
            $crate::span_assert!($span, $($arg)*);
        }
    };
}

/// A variant of [`std::debug_assert_eq!`] with better error messages.
#[macro_export]
macro_rules! debug_assert_eq {
    ($($arg:tt)*) => {
        if ::std::cfg!(debug_assertions) {
            $crate::assert_eq!($($arg)*);
        }
    };
}

/// A variant of [`std::debug_assert_eq!`] with better error messages emitted to a specific
/// [`Span`](rustc_span::Span).
#[macro_export]
macro_rules! debug_span_assert_eq {
    ($span:expr, $($arg:tt)*) => {
        if ::std::cfg!(debug_assertions) {
            $crate::span_assert_eq!($span, $($arg)*);
        }
    };
}

/// A variant of [`std::unreachable!`] with better error messages.
#[macro_export]
macro_rules! unreachable {
    () => {
        $crate::panic!("entered unreachable code")
    };
    ($($arg:tt)+) => {
        match ::std::format_args!($($arg)+) {
            message => {
                $crate::panic!("entered unreachable code: {message}")
            },
        }
    };
}

/// A variant of [`std::unreachable!`] with better error messages emitted to a specific
/// [`Span`](rustc_span::Span).
#[macro_export]
macro_rules! span_unreachable {
    ($span:expr) => {
        $crate::span_panic!($span, "entered unreachable code")
    };
    ($span:expr, $($arg:tt)+) => {
        match ::std::format_args!($($arg)+) {
            message => {
                $crate::span_panic!($span, "entered unreachable code: {message}")
            },
        }
    };
}
