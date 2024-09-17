use rustc_lint::Lint;

pub mod main_return_without_appexit;

pub static LINTS: &[&Lint] = &[main_return_without_appexit::MAIN_RETURN_WITHOUT_APPEXIT];
