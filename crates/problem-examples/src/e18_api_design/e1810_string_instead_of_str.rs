/// E1810: String instead of &str
/// Severity: LOW
/// LLM confusion: 2 (LOW)
///
/// Description: Accepting `String` as a parameter forces callers to own the string, requiring
/// allocation even if they have a string literal or `&str`. This is unnecessarily restrictive.
/// Fix by accepting `&str` instead, which works with string literals, `&str`, and `&String`
/// (via deref coercion).
///
/// Mitigation: Use `#![warn(clippy::needless_pass_by_value)]` to catch this. Accept `&str` for
/// string parameters unless you need ownership. Use `impl AsRef<str>` or `impl Into<String>` for
/// maximum flexibility. Only take `String` if you need to store or modify it.

pub fn e1810_string_instead_of_str(s: String) {
    // PROBLEM E1810: Forces caller to allocate, should accept &str
    println!("{s}");
}

pub fn e1810_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
