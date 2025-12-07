/// E1805: Missing documentation
/// Severity: LOW
/// LLM confusion: 1 (LOW)
///
/// Description: Public API functions should have documentation comments explaining what they do,
/// their parameters, return values, and any important behavior. This public function has no
/// documentation, making it hard for users to understand how to use it. Fix by adding doc comments
/// with `///` describing the function.
///
/// Mitigation: Use `#![warn(missing_docs)]` to require documentation on all public items. Write
/// doc comments with `///` for functions, `//!` for modules. Include examples in doc comments -
/// they're tested by `cargo test`. Good documentation is part of a good API.

pub fn e1805_missing_docs(x: i32) -> i32 {
    // PROBLEM E1805: No doc comment on public function
    x * 2
}

pub fn e1805_entry() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
