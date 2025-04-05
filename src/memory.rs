//! Utils for memory management.

/// Create a static reference by leaking the memory.
///
/// # Performance
///
/// Be careful with using this function in order to not exhaust the system's memory.
/// It should only be used when the string is expected to live until the end of the program anyway.
pub(crate) fn leak_to_static(s: &str) -> &'static str {
    Box::leak(s.to_owned().into_boxed_str())
}
